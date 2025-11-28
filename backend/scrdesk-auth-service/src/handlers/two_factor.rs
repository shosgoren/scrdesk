use axum::{extract::State, http::{HeaderMap, StatusCode}, Json};
use scrdesk_shared::error::{Error, Result};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::sync::Arc;
use totp_rs::{Algorithm, Secret, TOTP};

use crate::AppState;

#[derive(Debug, Serialize)]
pub struct Enable2FAResponse {
    pub secret: String,
    pub qr_code: String,
    pub backup_codes: Vec<String>,
}

pub async fn enable_2fa(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> Result<(StatusCode, Json<Enable2FAResponse>)> {
    // Get current user
    let auth_header = headers
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| Error::Authentication("Missing authorization header".to_string()))?;

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or_else(|| Error::Authentication("Invalid authorization header".to_string()))?;

    let claims = state.jwt_manager.verify_access_token(token)?;

    // Fetch user
    let user = sqlx::query_as::<_, scrdesk_shared::models::user::User>(
        "SELECT * FROM users WHERE id = $1"
    )
    .bind(claims.sub)
    .fetch_one(&state.db_pool)
    .await?;

    if user.two_factor_enabled {
        return Err(Error::Validation("2FA is already enabled".to_string()));
    }

    // Generate TOTP secret
    let secret = Secret::generate_secret();
    let secret_str = secret.to_encoded().to_string();

    let totp = TOTP::new(
        Algorithm::SHA1,
        6,
        1,
        30,
        secret.to_bytes().unwrap(),
        Some("ScrDesk".to_string()),
        user.email.clone(),
    )
    .map_err(|e| Error::Internal(e.to_string()))?;

    // Generate QR code
    let qr_code_url = totp.get_qr_base64()
        .map_err(|e| Error::Internal(e.to_string()))?;

    // Generate backup codes
    let backup_codes: Vec<String> = (0..10)
        .map(|_| format!("{:08}", rand::random::<u32>() % 100000000))
        .collect();

    // Store secret (not enabled yet)
    sqlx::query("UPDATE users SET two_factor_secret = $1 WHERE id = $2")
        .bind(&secret_str)
        .bind(user.id)
        .execute(&state.db_pool)
        .await?;

    // TODO: Store backup codes in database

    Ok((
        StatusCode::OK,
        Json(Enable2FAResponse {
            secret: secret_str,
            qr_code: qr_code_url,
            backup_codes,
        }),
    ))
}

#[derive(Debug, Deserialize)]
pub struct Verify2FARequest {
    pub code: String,
}

pub async fn verify_2fa(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(payload): Json<Verify2FARequest>,
) -> Result<(StatusCode, Json<Value>)> {
    // Get current user
    let auth_header = headers
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| Error::Authentication("Missing authorization header".to_string()))?;

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or_else(|| Error::Authentication("Invalid authorization header".to_string()))?;

    let claims = state.jwt_manager.verify_access_token(token)?;

    // Fetch user
    let user = sqlx::query_as::<_, scrdesk_shared::models::user::User>(
        "SELECT * FROM users WHERE id = $1"
    )
    .bind(claims.sub)
    .fetch_one(&state.db_pool)
    .await?;

    let secret = user.two_factor_secret
        .ok_or_else(|| Error::Validation("2FA not set up".to_string()))?;

    // Verify TOTP code
    let totp = TOTP::new(
        Algorithm::SHA1,
        6,
        1,
        30,
        secret.as_bytes().to_vec(),
    )
    .map_err(|e| Error::Internal(e.to_string()))?;

    if !totp.check_current(&payload.code).unwrap_or(false) {
        return Err(Error::Authentication("Invalid 2FA code".to_string()));
    }

    // Enable 2FA
    sqlx::query("UPDATE users SET two_factor_enabled = true WHERE id = $1")
        .bind(user.id)
        .execute(&state.db_pool)
        .await?;

    Ok((
        StatusCode::OK,
        Json(json!({
            "message": "2FA enabled successfully"
        })),
    ))
}

pub async fn disable_2fa(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(payload): Json<Verify2FARequest>,
) -> Result<(StatusCode, Json<Value>)> {
    // Get current user
    let auth_header = headers
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| Error::Authentication("Missing authorization header".to_string()))?;

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or_else(|| Error::Authentication("Invalid authorization header".to_string()))?;

    let claims = state.jwt_manager.verify_access_token(token)?;

    // Fetch user
    let user = sqlx::query_as::<_, scrdesk_shared::models::user::User>(
        "SELECT * FROM users WHERE id = $1"
    )
    .bind(claims.sub)
    .fetch_one(&state.db_pool)
    .await?;

    if !user.two_factor_enabled {
        return Err(Error::Validation("2FA is not enabled".to_string()));
    }

    let secret = user.two_factor_secret.unwrap_or_default();

    // Verify TOTP code
    let totp = TOTP::new(
        Algorithm::SHA1,
        6,
        1,
        30,
        secret.as_bytes().to_vec(),
    )
    .map_err(|e| Error::Internal(e.to_string()))?;

    if !totp.check_current(&payload.code).unwrap_or(false) {
        return Err(Error::Authentication("Invalid 2FA code".to_string()));
    }

    // Disable 2FA
    sqlx::query("UPDATE users SET two_factor_enabled = false, two_factor_secret = NULL WHERE id = $1")
        .bind(user.id)
        .execute(&state.db_pool)
        .await?;

    Ok((
        StatusCode::OK,
        Json(json!({
            "message": "2FA disabled successfully"
        })),
    ))
}
