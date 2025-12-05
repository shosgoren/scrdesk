use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};
use chrono::Utc;
use oauth2::{
    basic::BasicClient, reqwest::async_http_client, AuthUrl, AuthorizationCode, ClientId,
    ClientSecret, CsrfToken, RedirectUrl, Scope, TokenResponse, TokenUrl,
};
use scrdesk_shared::{
    error::{Error, Result},
    models::{
        user::{AuthResponse, UserResponse},
        TenantId, UserRole,
    },
    utils::hash_password,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sqlx::Row;
use std::sync::Arc;
use uuid::Uuid;

use crate::AppState;

#[derive(Debug, Deserialize)]
pub struct OAuthCallbackQuery {
    pub code: String,
    pub state: String,
}

#[derive(Debug, Serialize)]
pub struct OAuthUrlResponse {
    pub url: String,
    pub state: String,
}

#[derive(Debug, Deserialize)]
pub struct GoogleUserInfo {
    pub id: String,
    pub email: String,
    pub name: String,
    pub picture: Option<String>,
    pub verified_email: bool,
}

#[derive(Debug, Deserialize)]
pub struct AppleUserInfo {
    pub sub: String,
    pub email: String,
}

// Google OAuth - Get authorization URL
pub async fn google_auth_url(
    State(state): State<Arc<AppState>>,
) -> Result<(StatusCode, Json<OAuthUrlResponse>)> {
    let google_client_id = std::env::var("GOOGLE_CLIENT_ID")
        .map_err(|_| Error::Internal("GOOGLE_CLIENT_ID not configured".to_string()))?;

    let google_client_secret = std::env::var("GOOGLE_CLIENT_SECRET")
        .map_err(|_| Error::Internal("GOOGLE_CLIENT_SECRET not configured".to_string()))?;

    let redirect_url = std::env::var("GOOGLE_REDIRECT_URL")
        .unwrap_or_else(|_| "https://scrdesk.com/api/v1/auth/oauth/google/callback".to_string());

    let client = BasicClient::new(
        ClientId::new(google_client_id),
        Some(ClientSecret::new(google_client_secret)),
        AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())
            .map_err(|e| Error::Internal(format!("Invalid auth URL: {}", e)))?,
        Some(
            TokenUrl::new("https://oauth2.googleapis.com/token".to_string())
                .map_err(|e| Error::Internal(format!("Invalid token URL: {}", e)))?,
        ),
    )
    .set_redirect_uri(
        RedirectUrl::new(redirect_url)
            .map_err(|e| Error::Internal(format!("Invalid redirect URL: {}", e)))?,
    );

    let (authorize_url, csrf_state) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("email".to_string()))
        .add_scope(Scope::new("profile".to_string()))
        .url();

    // Store CSRF token in Redis with 10 minute expiration
    let mut conn = state
        .redis_client
        .get_multiplexed_async_connection()
        .await
        .map_err(|e| Error::Internal(e.to_string()))?;

    redis::cmd("SETEX")
        .arg(format!("oauth:csrf:{}", csrf_state.secret()))
        .arg(600) // 10 minutes
        .arg("valid")
        .query_async::<_, ()>(&mut conn)
        .await
        .map_err(|e| Error::Internal(e.to_string()))?;

    Ok((
        StatusCode::OK,
        Json(OAuthUrlResponse {
            url: authorize_url.to_string(),
            state: csrf_state.secret().to_string(),
        }),
    ))
}

// Google OAuth - Callback handler
pub async fn google_callback(
    State(state): State<Arc<AppState>>,
    Query(params): Query<OAuthCallbackQuery>,
) -> Result<(StatusCode, Json<AuthResponse>)> {
    // Verify CSRF token
    let mut conn = state
        .redis_client
        .get_multiplexed_async_connection()
        .await
        .map_err(|e| Error::Internal(e.to_string()))?;

    let csrf_valid: Option<String> = redis::cmd("GET")
        .arg(format!("oauth:csrf:{}", params.state))
        .query_async(&mut conn)
        .await
        .map_err(|e| Error::Internal(e.to_string()))?;

    if csrf_valid.is_none() {
        return Err(Error::Authentication("Invalid or expired CSRF token".to_string()));
    }

    // Delete CSRF token
    redis::cmd("DEL")
        .arg(format!("oauth:csrf:{}", params.state))
        .query_async::<_, ()>(&mut conn)
        .await
        .map_err(|e| Error::Internal(e.to_string()))?;

    let google_client_id = std::env::var("GOOGLE_CLIENT_ID")
        .map_err(|_| Error::Internal("GOOGLE_CLIENT_ID not configured".to_string()))?;

    let google_client_secret = std::env::var("GOOGLE_CLIENT_SECRET")
        .map_err(|_| Error::Internal("GOOGLE_CLIENT_SECRET not configured".to_string()))?;

    let redirect_url = std::env::var("GOOGLE_REDIRECT_URL")
        .unwrap_or_else(|_| "https://scrdesk.com/api/v1/auth/oauth/google/callback".to_string());

    let client = BasicClient::new(
        ClientId::new(google_client_id),
        Some(ClientSecret::new(google_client_secret)),
        AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())
            .map_err(|e| Error::Internal(format!("Invalid auth URL: {}", e)))?,
        Some(
            TokenUrl::new("https://oauth2.googleapis.com/token".to_string())
                .map_err(|e| Error::Internal(format!("Invalid token URL: {}", e)))?,
        ),
    )
    .set_redirect_uri(
        RedirectUrl::new(redirect_url)
            .map_err(|e| Error::Internal(format!("Invalid redirect URL: {}", e)))?,
    );

    // Exchange authorization code for access token
    let token_response = client
        .exchange_code(AuthorizationCode::new(params.code))
        .request_async(async_http_client)
        .await
        .map_err(|e| Error::Internal(format!("Failed to exchange code: {}", e)))?;

    // Get user info from Google
    let user_info_response = reqwest::Client::new()
        .get("https://www.googleapis.com/oauth2/v2/userinfo")
        .bearer_auth(token_response.access_token().secret())
        .send()
        .await
        .map_err(|e| Error::Internal(format!("Failed to get user info: {}", e)))?;

    let google_user: GoogleUserInfo = user_info_response
        .json()
        .await
        .map_err(|e| Error::Internal(format!("Failed to parse user info: {}", e)))?;

    // Check if user exists
    let existing_user = sqlx::query_as::<_, scrdesk_shared::models::user::User>(
        "SELECT * FROM users WHERE email = $1",
    )
    .bind(&google_user.email)
    .fetch_optional(&state.db_pool)
    .await?;

    let user = if let Some(user) = existing_user {
        // Update last login
        sqlx::query("UPDATE users SET last_login_at = $1 WHERE id = $2")
            .bind(Utc::now())
            .bind(user.id)
            .execute(&state.db_pool)
            .await?;
        user
    } else {
        // Create new user
        let slug = scrdesk_shared::utils::slugify(&google_user.email);
        let tenant_id: TenantId = sqlx::query(
            "INSERT INTO tenants (name, slug, plan) VALUES ($1, $2, $3::plan_type) RETURNING id",
        )
        .bind(&google_user.name)
        .bind(&slug)
        .bind("free")
        .fetch_one(&state.db_pool)
        .await?
        .get(0);

        // Random password for OAuth users (they won't use it)
        let random_password = Uuid::new_v4().to_string();
        let password_hash = hash_password(&random_password)?;

        let user_id: Uuid = sqlx::query_scalar(
            "INSERT INTO users (tenant_id, email, password_hash, full_name, role, is_email_verified)
             VALUES ($1, $2, $3, $4, $5, $6) RETURNING id",
        )
        .bind(tenant_id)
        .bind(&google_user.email)
        .bind(&password_hash)
        .bind(&google_user.name)
        .bind(UserRole::User)
        .bind(google_user.verified_email)
        .fetch_one(&state.db_pool)
        .await?;

        sqlx::query_as::<_, scrdesk_shared::models::user::User>("SELECT * FROM users WHERE id = $1")
            .bind(user_id)
            .fetch_one(&state.db_pool)
            .await?
    };

    // Generate JWT tokens
    let access_token = state.jwt_manager.generate_access_token(
        user.id,
        user.tenant_id,
        user.role,
    )?;

    let refresh_token = state.jwt_manager.generate_refresh_token(
        user.id,
        user.tenant_id,
        user.role,
    )?;

    // Store refresh token
    let token_hash = hash_password(&refresh_token)?;
    let expires_at = Utc::now() + chrono::Duration::seconds(state.config.jwt.refresh_token_expiry);

    sqlx::query(
        "INSERT INTO refresh_tokens (user_id, token_hash, expires_at) VALUES ($1, $2, $3)",
    )
    .bind(user.id)
    .bind(&token_hash)
    .bind(expires_at)
    .execute(&state.db_pool)
    .await?;

    Ok((
        StatusCode::OK,
        Json(AuthResponse {
            access_token,
            refresh_token,
            expires_in: state.config.jwt.access_token_expiry,
            user: user.into(),
        }),
    ))
}

// Apple OAuth - Get authorization URL
pub async fn apple_auth_url(
    State(state): State<Arc<AppState>>,
) -> Result<(StatusCode, Json<OAuthUrlResponse>)> {
    let apple_client_id = std::env::var("APPLE_CLIENT_ID")
        .map_err(|_| Error::Internal("APPLE_CLIENT_ID not configured".to_string()))?;

    let redirect_url = std::env::var("APPLE_REDIRECT_URL")
        .unwrap_or_else(|_| "https://scrdesk.com/api/v1/auth/oauth/apple/callback".to_string());

    let client = BasicClient::new(
        ClientId::new(apple_client_id),
        None, // Apple uses JWT instead of client secret
        AuthUrl::new("https://appleid.apple.com/auth/authorize".to_string())
            .map_err(|e| Error::Internal(format!("Invalid auth URL: {}", e)))?,
        Some(
            TokenUrl::new("https://appleid.apple.com/auth/token".to_string())
                .map_err(|e| Error::Internal(format!("Invalid token URL: {}", e)))?,
        ),
    )
    .set_redirect_uri(
        RedirectUrl::new(redirect_url)
            .map_err(|e| Error::Internal(format!("Invalid redirect URL: {}", e)))?,
    );

    let (authorize_url, csrf_state) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("email".to_string()))
        .add_scope(Scope::new("name".to_string()))
        .url();

    // Store CSRF token in Redis
    let mut conn = state
        .redis_client
        .get_multiplexed_async_connection()
        .await
        .map_err(|e| Error::Internal(e.to_string()))?;

    redis::cmd("SETEX")
        .arg(format!("oauth:csrf:{}", csrf_state.secret()))
        .arg(600)
        .arg("valid")
        .query_async::<_, ()>(&mut conn)
        .await
        .map_err(|e| Error::Internal(e.to_string()))?;

    Ok((
        StatusCode::OK,
        Json(OAuthUrlResponse {
            url: authorize_url.to_string(),
            state: csrf_state.secret().to_string(),
        }),
    ))
}
