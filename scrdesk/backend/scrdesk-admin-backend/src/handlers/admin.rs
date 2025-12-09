use axum::{extract::{Path, Query, State}, http::{HeaderMap, StatusCode}, Json};
use scrdesk_shared::{error::{Error, Result}, models::PaginationParams};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;
use uuid::Uuid;
use crate::AppState;

#[derive(Debug, Serialize)]
pub struct DashboardStats {
    pub total_tenants: i64,
    pub total_users: i64,
    pub total_devices: i64,
    pub active_sessions: i64,
    pub total_sessions_today: i64,
}

pub async fn get_dashboard_stats(State(state): State<Arc<AppState>>, headers: HeaderMap) -> Result<(StatusCode, Json<DashboardStats>)> {
    let auth_header = headers.get("Authorization").and_then(|h| h.to_str().ok())
        .ok_or_else(|| Error::Authentication("Missing authorization header".to_string()))?;
    let token = auth_header.strip_prefix("Bearer ")
        .ok_or_else(|| Error::Authentication("Invalid authorization header".to_string()))?;
    let claims = state.jwt_manager.verify_access_token(token)?;

    // Check if super admin
    if claims.role != scrdesk_shared::models::UserRole::SuperAdmin {
        return Err(Error::Authorization("Super admin access required".to_string()));
    }

    let total_tenants: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM tenants").fetch_one(&state.db_pool).await?;
    let total_users: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users").fetch_one(&state.db_pool).await?;
    let total_devices: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM devices").fetch_one(&state.db_pool).await?;
    let active_sessions: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM sessions WHERE ended_at IS NULL").fetch_one(&state.db_pool).await?;
    let total_sessions_today: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM sessions WHERE started_at >= CURRENT_DATE").fetch_one(&state.db_pool).await?;

    Ok((StatusCode::OK, Json(DashboardStats { total_tenants, total_users, total_devices, active_sessions, total_sessions_today })))
}

pub async fn list_all_tenants(State(state): State<Arc<AppState>>, headers: HeaderMap, Query(pagination): Query<PaginationParams>) -> Result<(StatusCode, Json<Value>)> {
    let auth_header = headers.get("Authorization").and_then(|h| h.to_str().ok())
        .ok_or_else(|| Error::Authentication("Missing authorization header".to_string()))?;
    let token = auth_header.strip_prefix("Bearer ")
        .ok_or_else(|| Error::Authentication("Invalid authorization header".to_string()))?;
    let claims = state.jwt_manager.verify_access_token(token)?;

    if claims.role != scrdesk_shared::models::UserRole::SuperAdmin {
        return Err(Error::Authorization("Super admin access required".to_string()));
    }

    let tenants = sqlx::query_as::<_, scrdesk_shared::models::tenant::Tenant>(
        "SELECT * FROM tenants ORDER BY created_at DESC LIMIT $1 OFFSET $2")
        .bind(pagination.limit() as i32).bind(pagination.offset() as i32)
        .fetch_all(&state.db_pool).await?;

    Ok((StatusCode::OK, Json(serde_json::to_value(tenants)?)))
}

pub async fn list_all_users(State(state): State<Arc<AppState>>, headers: HeaderMap) -> Result<(StatusCode, Json<Value>)> {
    let auth_header = headers.get("Authorization").and_then(|h| h.to_str().ok())
        .ok_or_else(|| Error::Authentication("Missing authorization header".to_string()))?;
    let token = auth_header.strip_prefix("Bearer ")
        .ok_or_else(|| Error::Authentication("Invalid authorization header".to_string()))?;
    let claims = state.jwt_manager.verify_access_token(token)?;

    if claims.role != scrdesk_shared::models::UserRole::SuperAdmin {
        return Err(Error::Authorization("Super admin access required".to_string()));
    }

    let users = sqlx::query_as::<_, scrdesk_shared::models::user::User>(
        "SELECT * FROM users ORDER BY created_at DESC LIMIT 1000")
        .fetch_all(&state.db_pool).await?;

    Ok((StatusCode::OK, Json(serde_json::to_value(users)?)))
}

pub async fn list_active_sessions(State(state): State<Arc<AppState>>, headers: HeaderMap) -> Result<(StatusCode, Json<Value>)> {
    let auth_header = headers.get("Authorization").and_then(|h| h.to_str().ok())
        .ok_or_else(|| Error::Authentication("Missing authorization header".to_string()))?;
    let token = auth_header.strip_prefix("Bearer ")
        .ok_or_else(|| Error::Authentication("Invalid authorization header".to_string()))?;
    let claims = state.jwt_manager.verify_access_token(token)?;

    let sessions = sqlx::query_as::<_, scrdesk_shared::models::session::Session>(
        "SELECT * FROM sessions WHERE ended_at IS NULL ORDER BY started_at DESC LIMIT 100")
        .fetch_all(&state.db_pool).await?;

    Ok((StatusCode::OK, Json(serde_json::to_value(sessions)?)))
}

pub async fn get_session_recording(State(state): State<Arc<AppState>>, headers: HeaderMap, Path(id): Path<Uuid>) -> Result<(StatusCode, Json<Value>)> {
    let auth_header = headers.get("Authorization").and_then(|h| h.to_str().ok())
        .ok_or_else(|| Error::Authentication("Missing authorization header".to_string()))?;
    let token = auth_header.strip_prefix("Bearer ")
        .ok_or_else(|| Error::Authentication("Invalid authorization header".to_string()))?;
    let claims = state.jwt_manager.verify_access_token(token)?;

    let session = sqlx::query_as::<_, scrdesk_shared::models::session::Session>(
        "SELECT * FROM sessions WHERE id = $1").bind(id).fetch_optional(&state.db_pool).await?
        .ok_or_else(|| Error::NotFound("Session not found".to_string()))?;

    // TODO: Generate S3 presigned URL for recording
    Ok((StatusCode::OK, Json(serde_json::json!({
        "session_id": session.id,
        "recording_url": session.recording_url,
        "message": "Recording URL (presigned S3 URL would be here)"
    }))))
}
