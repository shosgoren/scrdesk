use axum::{
    extract::{Query, State},
    http::{HeaderMap, StatusCode},
    Json,
};
use chrono::Utc;
use scrdesk_shared::{
    error::{Error, Result},
    models::{
        user::{AuthResponse, CreateUserRequest, LoginRequest, UserResponse},
        TenantId, UserRole,
    },
    utils::{hash_password, verify_password},
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sqlx::Row;
use std::sync::Arc;
use uuid::Uuid;
use validator::Validate;

use crate::AppState;

#[derive(Debug, Deserialize)]
pub struct RegisterQuery {
    pub tenant_id: Option<TenantId>,
}

pub async fn register(
    State(state): State<Arc<AppState>>,
    Query(query): Query<RegisterQuery>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<(StatusCode, Json<AuthResponse>)> {
    payload.validate().map_err(|e| Error::Validation(e.to_string()))?;

    // Get or create tenant
    let tenant_id = if let Some(tid) = query.tenant_id {
        tid
    } else {
        // Create default tenant for first user
        let slug = scrdesk_shared::utils::slugify(&payload.email);
        let tenant_id = sqlx::query(
            "INSERT INTO tenants (name, slug, plan) VALUES ($1, $2, $3::plan_type) RETURNING id"
        )
        .bind(&payload.email)
        .bind(&slug)
        .bind("free")
        .fetch_one(&state.db_pool)
        .await?
        .get(0);

        tenant_id
    };

    // Hash password
    let password_hash = hash_password(&payload.password)?;

    // Insert user
    let user_id: Uuid = sqlx::query_scalar(
        "INSERT INTO users (tenant_id, email, password_hash, full_name, role)
         VALUES ($1, $2, $3, $4, $5) RETURNING id"
    )
    .bind(tenant_id)
    .bind(&payload.email)
    .bind(&password_hash)
    .bind(&payload.full_name)
    .bind(&payload.role)
    .fetch_one(&state.db_pool)
    .await
    .map_err(|e| {
        if e.to_string().contains("duplicate key") {
            Error::Validation("Email already exists".to_string())
        } else {
            Error::Database(e)
        }
    })?;

    // Fetch created user
    let user = sqlx::query_as::<_, scrdesk_shared::models::user::User>(
        "SELECT * FROM users WHERE id = $1"
    )
    .bind(user_id)
    .fetch_one(&state.db_pool)
    .await?;

    // Generate tokens
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
        "INSERT INTO refresh_tokens (user_id, token_hash, expires_at) VALUES ($1, $2, $3)"
    )
    .bind(user.id)
    .bind(&token_hash)
    .bind(expires_at)
    .execute(&state.db_pool)
    .await?;

    Ok((
        StatusCode::CREATED,
        Json(AuthResponse {
            access_token,
            refresh_token,
            expires_in: state.config.jwt.access_token_expiry,
            user: user.into(),
        }),
    ))
}

pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<LoginRequest>,
) -> Result<(StatusCode, Json<AuthResponse>)> {
    payload.validate().map_err(|e| Error::Validation(e.to_string()))?;

    // Fetch user
    let user = sqlx::query_as::<_, scrdesk_shared::models::user::User>(
        "SELECT * FROM users WHERE email = $1"
    )
    .bind(&payload.email)
    .fetch_optional(&state.db_pool)
    .await?
    .ok_or_else(|| Error::Authentication("Invalid credentials".to_string()))?;

    // Verify password
    if !verify_password(&payload.password, &user.password_hash)? {
        return Err(Error::Authentication("Invalid credentials".to_string()));
    }

    // Check if account is active
    if !user.is_active {
        return Err(Error::Authentication("Account is disabled".to_string()));
    }

    // Check 2FA
    if user.two_factor_enabled {
        if let Some(code) = payload.two_factor_code {
            // Verify 2FA code
            let secret = user.two_factor_secret.as_ref().map(|s| s.as_str()).unwrap_or("");
            if !verify_totp(secret, &code) {
                return Err(Error::Authentication("Invalid 2FA code".to_string()));
            }
        } else {
            return Err(Error::Authentication("2FA code required".to_string()));
        }
    }

    // Update last login
    sqlx::query("UPDATE users SET last_login_at = $1 WHERE id = $2")
        .bind(Utc::now())
        .bind(user.id)
        .execute(&state.db_pool)
        .await?;

    // Generate tokens
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
        "INSERT INTO refresh_tokens (user_id, token_hash, expires_at) VALUES ($1, $2, $3)"
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

#[derive(Debug, Deserialize)]
pub struct RefreshTokenRequest {
    pub refresh_token: String,
}

pub async fn refresh_token(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<RefreshTokenRequest>,
) -> Result<(StatusCode, Json<Value>)> {
    // Verify refresh token
    let claims = state.jwt_manager.verify_refresh_token(&payload.refresh_token)?;

    // Check if token exists in database
    let token_hash = hash_password(&payload.refresh_token)?;
    let exists: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM refresh_tokens WHERE token_hash = $1 AND revoked_at IS NULL AND expires_at > NOW())"
    )
    .bind(&token_hash)
    .fetch_one(&state.db_pool)
    .await?;

    if !exists {
        return Err(Error::Authentication("Invalid refresh token".to_string()));
    }

    // Generate new access token
    let access_token = state.jwt_manager.generate_access_token(
        claims.sub,
        claims.tenant_id,
        claims.role,
    )?;

    Ok((
        StatusCode::OK,
        Json(json!({
            "access_token": access_token,
            "expires_in": state.config.jwt.access_token_expiry,
        })),
    ))
}

pub async fn logout(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> Result<(StatusCode, Json<Value>)> {
    // Extract token from header
    let auth_header = headers
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| Error::Authentication("Missing authorization header".to_string()))?;

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or_else(|| Error::Authentication("Invalid authorization header".to_string()))?;

    // Verify token
    let claims = state.jwt_manager.verify_access_token(token)?;

    // Revoke all refresh tokens for this user
    sqlx::query("UPDATE refresh_tokens SET revoked_at = NOW() WHERE user_id = $1")
        .bind(claims.sub)
        .execute(&state.db_pool)
        .await?;

    Ok((
        StatusCode::OK,
        Json(json!({
            "message": "Logged out successfully"
        })),
    ))
}

pub async fn get_current_user(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> Result<(StatusCode, Json<UserResponse>)> {
    // Extract token from header
    let auth_header = headers
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| Error::Authentication("Missing authorization header".to_string()))?;

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or_else(|| Error::Authentication("Invalid authorization header".to_string()))?;

    // Verify token
    let claims = state.jwt_manager.verify_access_token(token)?;

    // Fetch user
    let user = sqlx::query_as::<_, scrdesk_shared::models::user::User>(
        "SELECT * FROM users WHERE id = $1"
    )
    .bind(claims.sub)
    .fetch_optional(&state.db_pool)
    .await?
    .ok_or_else(|| Error::NotFound("User not found".to_string()))?;

    Ok((StatusCode::OK, Json(user.into())))
}

#[derive(Debug, Deserialize)]
pub struct ChangePasswordRequest {
    pub old_password: String,
    pub new_password: String,
}

pub async fn change_password(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(payload): Json<ChangePasswordRequest>,
) -> Result<(StatusCode, Json<Value>)> {
    // Manual validation
    if payload.old_password.len() < 8 {
        return Err(Error::Validation("Old password must be at least 8 characters".to_string()));
    }
    if payload.new_password.len() < 8 {
        return Err(Error::Validation("New password must be at least 8 characters".to_string()));
    }

    // Get current user
    let auth_header = headers
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| Error::Authentication("Missing authorization header".to_string()))?;

    let token = auth_header.strip_prefix("Bearer ")
        .ok_or_else(|| Error::Authentication("Invalid authorization header".to_string()))?;

    let claims = state.jwt_manager.verify_access_token(token)?;

    // Fetch user
    let user = sqlx::query_as::<_, scrdesk_shared::models::user::User>(
        "SELECT * FROM users WHERE id = $1"
    )
    .bind(claims.sub)
    .fetch_one(&state.db_pool)
    .await?;

    // Verify old password
    if !verify_password(&payload.old_password, &user.password_hash)? {
        return Err(Error::Authentication("Invalid old password".to_string()));
    }

    // Hash new password
    let new_password_hash = hash_password(&payload.new_password)?;

    // Update password
    sqlx::query("UPDATE users SET password_hash = $1, updated_at = NOW() WHERE id = $2")
        .bind(&new_password_hash)
        .bind(user.id)
        .execute(&state.db_pool)
        .await?;

    // Revoke all refresh tokens
    sqlx::query("UPDATE refresh_tokens SET revoked_at = NOW() WHERE user_id = $1")
        .bind(user.id)
        .execute(&state.db_pool)
        .await?;

    Ok((
        StatusCode::OK,
        Json(json!({
            "message": "Password changed successfully"
        })),
    ))
}

#[derive(Debug, Deserialize)]
pub struct RequestPasswordResetRequest {
    pub email: String,
}

pub async fn request_password_reset(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<RequestPasswordResetRequest>,
) -> Result<(StatusCode, Json<Value>)> {
    // Basic email validation
    if !payload.email.contains('@') {
        return Err(Error::Validation("Invalid email address".to_string()));
    }

    // Check if user exists
    let user = sqlx::query_as::<_, scrdesk_shared::models::user::User>(
        "SELECT * FROM users WHERE email = $1"
    )
    .bind(&payload.email)
    .fetch_optional(&state.db_pool)
    .await?;

    if let Some(user) = user {
        // Generate reset token
        let reset_token = Uuid::new_v4().to_string();
        let token_hash = hash_password(&reset_token)?;
        let expires_at = Utc::now() + chrono::Duration::hours(1);

        // Store reset token (in production, add a password_reset_tokens table)
        let mut conn = state.redis_client.get_multiplexed_async_connection().await
            .map_err(|e| Error::Internal(e.to_string()))?;

        redis::cmd("SETEX")
            .arg(format!("password_reset:{}", user.id))
            .arg(3600) // 1 hour
            .arg(&token_hash)
            .query_async::<_, ()>(&mut conn)
            .await
            .map_err(|e| Error::Internal(e.to_string()))?;

        // TODO: Send email with reset link
        tracing::info!("Password reset requested for user: {}, token: {}", user.email, reset_token);
    }

    // Always return success to prevent email enumeration
    Ok((
        StatusCode::OK,
        Json(json!({
            "message": "If the email exists, a password reset link has been sent"
        })),
    ))
}

#[derive(Debug, Deserialize)]
pub struct ConfirmPasswordResetRequest {
    pub token: String,
    pub new_password: String,
}

pub async fn confirm_password_reset(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<ConfirmPasswordResetRequest>,
) -> Result<(StatusCode, Json<Value>)> {
    // Manual validation
    if payload.new_password.len() < 8 {
        return Err(Error::Validation("New password must be at least 8 characters".to_string()));
    }

    // TODO: Verify token from database/redis and update password
    // This is a simplified implementation

    Ok((
        StatusCode::OK,
        Json(json!({
            "message": "Password reset successfully"
        })),
    ))
}

fn verify_totp(secret: &str, code: &str) -> bool {
    use totp_rs::{Algorithm, TOTP};

    if secret.is_empty() {
        return false;
    }

    // Decode base32 secret
    let secret_bytes = match base32::decode(base32::Alphabet::Rfc4648 { padding: false }, secret) {
        Some(bytes) => bytes,
        None => return false,
    };

    let totp = match TOTP::new(
        Algorithm::SHA1,
        6,
        1,
        30,
        secret_bytes,
    ) {
        Ok(t) => t,
        Err(_) => return false,
    };

    totp.check_current(code).unwrap_or(false)
}
