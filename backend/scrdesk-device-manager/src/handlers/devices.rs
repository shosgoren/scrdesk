use axum::{
    extract::{Path, Query, State},
    http::{HeaderMap, StatusCode},
    Json,
};
use chrono::Utc;
use scrdesk_shared::{
    error::{Error, Result},
    models::{
        device::{DeviceResponse, DeviceStatus, RegisterDeviceRequest, UpdateDeviceRequest},
        DeviceId, PaginationParams, PaginatedResponse,
    },
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::sync::Arc;
use uuid::Uuid;
use validator::Validate;

use crate::AppState;

/// Register a new device
pub async fn register_device(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(payload): Json<RegisterDeviceRequest>,
) -> Result<(StatusCode, Json<DeviceResponse>)> {
    payload.validate().map_err(|e| Error::Validation(e.to_string()))?;

    // Get tenant from JWT token
    let auth_header = headers
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| Error::Authentication("Missing authorization header".to_string()))?;

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or_else(|| Error::Authentication("Invalid authorization header".to_string()))?;

    let claims = state.jwt_manager.verify_access_token(token)?;

    // Check device limit for tenant
    let tenant = sqlx::query_as::<_, scrdesk_shared::models::tenant::Tenant>(
        "SELECT * FROM tenants WHERE id = $1"
    )
    .bind(claims.tenant_id)
    .fetch_one(&state.db_pool)
    .await?;

    // Count existing devices
    let device_count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM devices WHERE tenant_id = $1"
    )
    .bind(claims.tenant_id)
    .fetch_one(&state.db_pool)
    .await?;

    if let Some(limit) = tenant.device_limit {
        if device_count >= limit as i64 {
            return Err(Error::DeviceLimitExceeded);
        }
    }

    // Insert device
    let device = sqlx::query_as::<_, scrdesk_shared::models::device::Device>(
        "INSERT INTO devices (tenant_id, owner_id, device_id, device_name, platform, os_version, client_version, public_key, status)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) RETURNING *"
    )
    .bind(claims.tenant_id)
    .bind(Some(claims.sub))
    .bind(&payload.device_id)
    .bind(&payload.device_name)
    .bind(&payload.platform)
    .bind(&payload.os_version)
    .bind(&payload.client_version)
    .bind(&payload.public_key)
    .bind(DeviceStatus::Offline)
    .fetch_one(&state.db_pool)
    .await
    .map_err(|e| {
        if e.to_string().contains("duplicate key") {
            Error::Validation("Device ID already exists".to_string())
        } else {
            Error::Database(e)
        }
    })?;

    // Log audit event
    sqlx::query(
        "INSERT INTO audit_logs (tenant_id, user_id, action, resource_type, resource_id)
         VALUES ($1, $2, $3, $4, $5)"
    )
    .bind(claims.tenant_id)
    .bind(claims.sub)
    .bind("DEVICE_REGISTERED")
    .bind("device")
    .bind(device.id)
    .execute(&state.db_pool)
    .await?;

    Ok((StatusCode::CREATED, Json(device.into())))
}

/// List all devices for tenant with pagination
pub async fn list_devices(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Query(pagination): Query<PaginationParams>,
) -> Result<(StatusCode, Json<PaginatedResponse<DeviceResponse>>)> {
    let auth_header = headers
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| Error::Authentication("Missing authorization header".to_string()))?;

    let token = auth_header.strip_prefix("Bearer ")
        .ok_or_else(|| Error::Authentication("Invalid authorization header".to_string()))?;

    let claims = state.jwt_manager.verify_access_token(token)?;

    // Get total count
    let total: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM devices WHERE tenant_id = $1"
    )
    .bind(claims.tenant_id)
    .fetch_one(&state.db_pool)
    .await?;

    // Get devices with pagination
    let devices = sqlx::query_as::<_, scrdesk_shared::models::device::Device>(
        "SELECT * FROM devices WHERE tenant_id = $1 ORDER BY created_at DESC LIMIT $2 OFFSET $3"
    )
    .bind(claims.tenant_id)
    .bind(pagination.limit() as i32)
    .bind(pagination.offset() as i32)
    .fetch_all(&state.db_pool)
    .await?;

    let device_responses: Vec<_> = devices.into_iter().map(|d| d.into()).collect();

    Ok((
        StatusCode::OK,
        Json(PaginatedResponse::new(device_responses, total, pagination)),
    ))
}

/// Get device by ID
pub async fn get_device(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path(id): Path<DeviceId>,
) -> Result<(StatusCode, Json<DeviceResponse>)> {
    let auth_header = headers
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| Error::Authentication("Missing authorization header".to_string()))?;

    let token = auth_header.strip_prefix("Bearer ")
        .ok_or_else(|| Error::Authentication("Invalid authorization header".to_string()))?;

    let claims = state.jwt_manager.verify_access_token(token)?;

    let device = sqlx::query_as::<_, scrdesk_shared::models::device::Device>(
        "SELECT * FROM devices WHERE id = $1 AND tenant_id = $2"
    )
    .bind(id)
    .bind(claims.tenant_id)
    .fetch_optional(&state.db_pool)
    .await?
    .ok_or_else(|| Error::NotFound("Device not found".to_string()))?;

    Ok((StatusCode::OK, Json(device.into())))
}

/// Update device
pub async fn update_device(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path(id): Path<DeviceId>,
    Json(payload): Json<UpdateDeviceRequest>,
) -> Result<(StatusCode, Json<DeviceResponse>)> {
    payload.validate().map_err(|e| Error::Validation(e.to_string()))?;

    let auth_header = headers
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| Error::Authentication("Missing authorization header".to_string()))?;

    let token = auth_header.strip_prefix("Bearer ")
        .ok_or_else(|| Error::Authentication("Invalid authorization header".to_string()))?;

    let claims = state.jwt_manager.verify_access_token(token)?;

    // Build dynamic update query
    let mut query = String::from("UPDATE devices SET ");
    let mut params: Vec<String> = Vec::new();
    let mut param_count = 1;

    if let Some(name) = &payload.device_name {
        params.push(format!("device_name = ${}", param_count));
        param_count += 1;
    }

    if let Some(status) = &payload.status {
        params.push(format!("status = ${}", param_count));
        param_count += 1;
    }

    if let Some(is_approved) = &payload.is_approved {
        params.push(format!("is_approved = ${}", param_count));
        param_count += 1;
    }

    params.push(format!("updated_at = NOW()"));

    query.push_str(&params.join(", "));
    query.push_str(&format!(" WHERE id = ${} AND tenant_id = ${} RETURNING *", param_count, param_count + 1));

    let mut query_builder = sqlx::query_as::<_, scrdesk_shared::models::device::Device>(&query);

    if let Some(name) = &payload.device_name {
        query_builder = query_builder.bind(name);
    }

    if let Some(status) = &payload.status {
        query_builder = query_builder.bind(status);
    }

    if let Some(is_approved) = &payload.is_approved {
        query_builder = query_builder.bind(is_approved);
    }

    let device = query_builder
        .bind(id)
        .bind(claims.tenant_id)
        .fetch_optional(&state.db_pool)
        .await?
        .ok_or_else(|| Error::NotFound("Device not found".to_string()))?;

    Ok((StatusCode::OK, Json(device.into())))
}

/// Delete device
pub async fn delete_device(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path(id): Path<DeviceId>,
) -> Result<(StatusCode, Json<Value>)> {
    let auth_header = headers
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| Error::Authentication("Missing authorization header".to_string()))?;

    let token = auth_header.strip_prefix("Bearer ")
        .ok_or_else(|| Error::Authentication("Invalid authorization header".to_string()))?;

    let claims = state.jwt_manager.verify_access_token(token)?;

    let result = sqlx::query("DELETE FROM devices WHERE id = $1 AND tenant_id = $2")
        .bind(id)
        .bind(claims.tenant_id)
        .execute(&state.db_pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(Error::NotFound("Device not found".to_string()));
    }

    // Log audit event
    sqlx::query(
        "INSERT INTO audit_logs (tenant_id, user_id, action, resource_type, resource_id)
         VALUES ($1, $2, $3, $4, $5)"
    )
    .bind(claims.tenant_id)
    .bind(claims.sub)
    .bind("DEVICE_REVOKED")
    .bind("device")
    .bind(id)
    .execute(&state.db_pool)
    .await?;

    Ok((
        StatusCode::OK,
        Json(json!({
            "message": "Device deleted successfully"
        })),
    ))
}

/// Approve device
pub async fn approve_device(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path(id): Path<DeviceId>,
) -> Result<(StatusCode, Json<DeviceResponse>)> {
    let auth_header = headers
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| Error::Authentication("Missing authorization header".to_string()))?;

    let token = auth_header.strip_prefix("Bearer ")
        .ok_or_else(|| Error::Authentication("Invalid authorization header".to_string()))?;

    let claims = state.jwt_manager.verify_access_token(token)?;

    let device = sqlx::query_as::<_, scrdesk_shared::models::device::Device>(
        "UPDATE devices SET is_approved = true, updated_at = NOW() WHERE id = $1 AND tenant_id = $2 RETURNING *"
    )
    .bind(id)
    .bind(claims.tenant_id)
    .fetch_optional(&state.db_pool)
    .await?
    .ok_or_else(|| Error::NotFound("Device not found".to_string()))?;

    // Log audit event
    sqlx::query(
        "INSERT INTO audit_logs (tenant_id, user_id, action, resource_type, resource_id)
         VALUES ($1, $2, $3, $4, $5)"
    )
    .bind(claims.tenant_id)
    .bind(claims.sub)
    .bind("DEVICE_APPROVED")
    .bind("device")
    .bind(id)
    .execute(&state.db_pool)
    .await?;

    Ok((StatusCode::OK, Json(device.into())))
}

/// Revoke device approval
pub async fn revoke_device(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path(id): Path<DeviceId>,
) -> Result<(StatusCode, Json<DeviceResponse>)> {
    let auth_header = headers
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| Error::Authentication("Missing authorization header".to_string()))?;

    let token = auth_header.strip_prefix("Bearer ")
        .ok_or_else(|| Error::Authentication("Invalid authorization header".to_string()))?;

    let claims = state.jwt_manager.verify_access_token(token)?;

    let device = sqlx::query_as::<_, scrdesk_shared::models::device::Device>(
        "UPDATE devices SET is_approved = false, status = 'offline', updated_at = NOW() WHERE id = $1 AND tenant_id = $2 RETURNING *"
    )
    .bind(id)
    .bind(claims.tenant_id)
    .fetch_optional(&state.db_pool)
    .await?
    .ok_or_else(|| Error::NotFound("Device not found".to_string()))?;

    Ok((StatusCode::OK, Json(device.into())))
}

#[derive(Debug, Deserialize)]
pub struct HeartbeatRequest {
    pub ip_address: Option<String>,
}

/// Device heartbeat to update last_seen_at
pub async fn device_heartbeat(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path(id): Path<DeviceId>,
    Json(payload): Json<HeartbeatRequest>,
) -> Result<(StatusCode, Json<Value>)> {
    let auth_header = headers
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| Error::Authentication("Missing authorization header".to_string()))?;

    let token = auth_header.strip_prefix("Bearer ")
        .ok_or_else(|| Error::Authentication("Invalid authorization header".to_string()))?;

    let claims = state.jwt_manager.verify_access_token(token)?;

    sqlx::query(
        "UPDATE devices SET last_seen_at = NOW(), ip_address = $1, status = 'online' WHERE id = $2 AND tenant_id = $3"
    )
    .bind(&payload.ip_address)
    .bind(id)
    .bind(claims.tenant_id)
    .execute(&state.db_pool)
    .await?;

    Ok((
        StatusCode::OK,
        Json(json!({
            "message": "Heartbeat received"
        })),
    ))
}

/// Update device status
pub async fn update_device_status(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path(id): Path<DeviceId>,
    Json(payload): Json<serde_json::Value>,
) -> Result<(StatusCode, Json<DeviceResponse>)> {
    let auth_header = headers
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| Error::Authentication("Missing authorization header".to_string()))?;

    let token = auth_header.strip_prefix("Bearer ")
        .ok_or_else(|| Error::Authentication("Invalid authorization header".to_string()))?;

    let claims = state.jwt_manager.verify_access_token(token)?;

    let status = payload.get("status")
        .and_then(|s| s.as_str())
        .ok_or_else(|| Error::Validation("Missing status field".to_string()))?;

    let device = sqlx::query_as::<_, scrdesk_shared::models::device::Device>(
        "UPDATE devices SET status = $1, updated_at = NOW() WHERE id = $2 AND tenant_id = $3 RETURNING *"
    )
    .bind(status)
    .bind(id)
    .bind(claims.tenant_id)
    .fetch_optional(&state.db_pool)
    .await?
    .ok_or_else(|| Error::NotFound("Device not found".to_string()))?;

    Ok((StatusCode::OK, Json(device.into())))
}

/// Get device groups
pub async fn get_device_groups(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path(id): Path<DeviceId>,
) -> Result<(StatusCode, Json<Vec<Uuid>>)> {
    let auth_header = headers
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| Error::Authentication("Missing authorization header".to_string()))?;

    let token = auth_header.strip_prefix("Bearer ")
        .ok_or_else(|| Error::Authentication("Invalid authorization header".to_string()))?;

    let claims = state.jwt_manager.verify_access_token(token)?;

    let group_ids: Vec<Uuid> = sqlx::query_scalar(
        "SELECT group_id FROM device_groups WHERE device_id = $1"
    )
    .bind(id)
    .fetch_all(&state.db_pool)
    .await?;

    Ok((StatusCode::OK, Json(group_ids)))
}

/// Add device to group
pub async fn add_device_to_group(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path((id, group_id)): Path<(DeviceId, Uuid)>,
) -> Result<(StatusCode, Json<Value>)> {
    let auth_header = headers
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| Error::Authentication("Missing authorization header".to_string()))?;

    let token = auth_header.strip_prefix("Bearer ")
        .ok_or_else(|| Error::Authentication("Invalid authorization header".to_string()))?;

    let claims = state.jwt_manager.verify_access_token(token)?;

    sqlx::query(
        "INSERT INTO device_groups (device_id, group_id) VALUES ($1, $2)"
    )
    .bind(id)
    .bind(group_id)
    .execute(&state.db_pool)
    .await
    .map_err(|e| {
        if e.to_string().contains("duplicate key") {
            Error::Validation("Device already in group".to_string())
        } else {
            Error::Database(e)
        }
    })?;

    Ok((
        StatusCode::CREATED,
        Json(json!({
            "message": "Device added to group"
        })),
    ))
}

/// Remove device from group
pub async fn remove_device_from_group(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path((id, group_id)): Path<(DeviceId, Uuid)>,
) -> Result<(StatusCode, Json<Value>)> {
    let auth_header = headers
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| Error::Authentication("Missing authorization header".to_string()))?;

    let token = auth_header.strip_prefix("Bearer ")
        .ok_or_else(|| Error::Authentication("Invalid authorization header".to_string()))?;

    let claims = state.jwt_manager.verify_access_token(token)?;

    let result = sqlx::query(
        "DELETE FROM device_groups WHERE device_id = $1 AND group_id = $2"
    )
    .bind(id)
    .bind(group_id)
    .execute(&state.db_pool)
    .await?;

    if result.rows_affected() == 0 {
        return Err(Error::NotFound("Device not in group".to_string()));
    }

    Ok((
        StatusCode::OK,
        Json(json!({
            "message": "Device removed from group"
        })),
    ))
}

#[derive(Debug, Deserialize)]
pub struct ConnectionRequest {
    pub target_device_id: String,
}

/// Request connection to another device
pub async fn request_connection(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path(id): Path<DeviceId>,
    Json(payload): Json<ConnectionRequest>,
) -> Result<(StatusCode, Json<Value>)> {
    let auth_header = headers
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| Error::Authentication("Missing authorization header".to_string()))?;

    let token = auth_header.strip_prefix("Bearer ")
        .ok_or_else(|| Error::Authentication("Invalid authorization header".to_string()))?;

    let claims = state.jwt_manager.verify_access_token(token)?;

    // Verify source device exists and is approved
    let source_device = sqlx::query_as::<_, scrdesk_shared::models::device::Device>(
        "SELECT * FROM devices WHERE id = $1 AND tenant_id = $2 AND is_approved = true"
    )
    .bind(id)
    .bind(claims.tenant_id)
    .fetch_optional(&state.db_pool)
    .await?
    .ok_or_else(|| Error::NotFound("Source device not found or not approved".to_string()))?;

    // Verify target device exists and is approved
    let target_device = sqlx::query_as::<_, scrdesk_shared::models::device::Device>(
        "SELECT * FROM devices WHERE device_id = $1 AND tenant_id = $2 AND is_approved = true"
    )
    .bind(&payload.target_device_id)
    .bind(claims.tenant_id)
    .fetch_optional(&state.db_pool)
    .await?
    .ok_or_else(|| Error::NotFound("Target device not found or not approved".to_string()))?;

    // TODO: Check policies
    // TODO: Create session record
    // TODO: Return relay server information

    Ok((
        StatusCode::OK,
        Json(json!({
            "message": "Connection request accepted",
            "target_device": {
                "id": target_device.id,
                "device_id": target_device.device_id,
                "public_key": target_device.public_key,
            },
            "relay_server": "relay.scrdesk.com:21117"  // TODO: Get from relay cluster
        })),
    ))
}
