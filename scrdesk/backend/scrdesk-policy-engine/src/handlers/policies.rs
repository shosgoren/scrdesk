use axum::{
    extract::{Path, Query, State},
    http::{HeaderMap, StatusCode},
    Json,
};
use scrdesk_shared::{
    error::{Error, Result},
    models::{
        policy::{Policy, PolicyResponse},
        PaginationParams, PaginatedResponse, PolicyId,
    },
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::sync::Arc;
use uuid::Uuid;
use validator::Validate;

use crate::AppState;

#[derive(Debug, Deserialize, Validate)]
pub struct CreatePolicyRequest {
    #[validate(length(min = 1, max = 100))]
    pub name: String,

    pub description: Option<String>,

    pub rules: scrdesk_shared::models::policy::PolicyRules,
}

pub async fn create_policy(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(payload): Json<CreatePolicyRequest>,
) -> Result<(StatusCode, Json<PolicyResponse>)> {
    payload.validate().map_err(|e| Error::Validation(e.to_string()))?;

    let auth_header = headers
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| Error::Authentication("Missing authorization header".to_string()))?;

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or_else(|| Error::Authentication("Invalid authorization header".to_string()))?;

    let claims = state.jwt_manager.verify_access_token(token)?;

    let policy = sqlx::query_as::<_, Policy>(
        "INSERT INTO policies (tenant_id, name, description, rules) VALUES ($1, $2, $3, $4) RETURNING *"
    )
    .bind(claims.tenant_id)
    .bind(&payload.name)
    .bind(&payload.description)
    .bind(serde_json::to_value(&payload.rules).unwrap())
    .fetch_one(&state.db_pool)
    .await?;

    // Log audit
    sqlx::query(
        "INSERT INTO audit_logs (tenant_id, user_id, action, resource_type, resource_id)
         VALUES ($1, $2, $3, $4, $5)"
    )
    .bind(claims.tenant_id)
    .bind(claims.sub)
    .bind("POLICY_CREATED")
    .bind("policy")
    .bind(policy.id)
    .execute(&state.db_pool)
    .await?;

    Ok((StatusCode::CREATED, Json(policy.into())))
}

pub async fn list_policies(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Query(pagination): Query<PaginationParams>,
) -> Result<(StatusCode, Json<PaginatedResponse<PolicyResponse>>)> {
    let auth_header = headers
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| Error::Authentication("Missing authorization header".to_string()))?;

    let token = auth_header.strip_prefix("Bearer ")
        .ok_or_else(|| Error::Authentication("Invalid authorization header".to_string()))?;

    let claims = state.jwt_manager.verify_access_token(token)?;

    let total: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM policies WHERE tenant_id = $1"
    )
    .bind(claims.tenant_id)
    .fetch_one(&state.db_pool)
    .await?;

    let policies = sqlx::query_as::<_, Policy>(
        "SELECT * FROM policies WHERE tenant_id = $1 ORDER BY created_at DESC LIMIT $2 OFFSET $3"
    )
    .bind(claims.tenant_id)
    .bind(pagination.limit() as i32)
    .bind(pagination.offset() as i32)
    .fetch_all(&state.db_pool)
    .await?;

    let policy_responses: Vec<_> = policies.into_iter().map(|p| p.into()).collect();

    Ok((
        StatusCode::OK,
        Json(PaginatedResponse::new(policy_responses, total, pagination)),
    ))
}

pub async fn get_policy(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path(id): Path<PolicyId>,
) -> Result<(StatusCode, Json<PolicyResponse>)> {
    let auth_header = headers
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| Error::Authentication("Missing authorization header".to_string()))?;

    let token = auth_header.strip_prefix("Bearer ")
        .ok_or_else(|| Error::Authentication("Invalid authorization header".to_string()))?;

    let claims = state.jwt_manager.verify_access_token(token)?;

    let policy = sqlx::query_as::<_, Policy>(
        "SELECT * FROM policies WHERE id = $1 AND tenant_id = $2"
    )
    .bind(id)
    .bind(claims.tenant_id)
    .fetch_optional(&state.db_pool)
    .await?
    .ok_or_else(|| Error::NotFound("Policy not found".to_string()))?;

    Ok((StatusCode::OK, Json(policy.into())))
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdatePolicyRequest {
    #[validate(length(min = 1, max = 100))]
    pub name: Option<String>,

    pub description: Option<String>,

    pub rules: Option<scrdesk_shared::models::policy::PolicyRules>,

    pub is_active: Option<bool>,
}

pub async fn update_policy(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path(id): Path<PolicyId>,
    Json(payload): Json<UpdatePolicyRequest>,
) -> Result<(StatusCode, Json<PolicyResponse>)> {
    payload.validate().map_err(|e| Error::Validation(e.to_string()))?;

    let auth_header = headers
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| Error::Authentication("Missing authorization header".to_string()))?;

    let token = auth_header.strip_prefix("Bearer ")
        .ok_or_else(|| Error::Authentication("Invalid authorization header".to_string()))?;

    let claims = state.jwt_manager.verify_access_token(token)?;

    // Build dynamic update
    let mut updates = vec![];
    let mut bind_count = 1;

    if payload.name.is_some() {
        updates.push(format!("name = ${}", bind_count));
        bind_count += 1;
    }

    if payload.description.is_some() {
        updates.push(format!("description = ${}", bind_count));
        bind_count += 1;
    }

    if payload.rules.is_some() {
        updates.push(format!("rules = ${}", bind_count));
        bind_count += 1;
    }

    if payload.is_active.is_some() {
        updates.push(format!("is_active = ${}", bind_count));
        bind_count += 1;
    }

    if updates.is_empty() {
        return Err(Error::Validation("No fields to update".to_string()));
    }

    updates.push("updated_at = NOW()".to_string());

    let query = format!(
        "UPDATE policies SET {} WHERE id = ${} AND tenant_id = ${} RETURNING *",
        updates.join(", "),
        bind_count,
        bind_count + 1
    );

    let mut query_builder = sqlx::query_as::<_, Policy>(&query);

    if let Some(name) = &payload.name {
        query_builder = query_builder.bind(name);
    }

    if let Some(description) = &payload.description {
        query_builder = query_builder.bind(description);
    }

    if let Some(rules) = &payload.rules {
        query_builder = query_builder.bind(serde_json::to_value(rules).unwrap());
    }

    if let Some(is_active) = &payload.is_active {
        query_builder = query_builder.bind(is_active);
    }

    let policy = query_builder
        .bind(id)
        .bind(claims.tenant_id)
        .fetch_optional(&state.db_pool)
        .await?
        .ok_or_else(|| Error::NotFound("Policy not found".to_string()))?;

    // Log audit
    sqlx::query(
        "INSERT INTO audit_logs (tenant_id, user_id, action, resource_type, resource_id)
         VALUES ($1, $2, $3, $4, $5)"
    )
    .bind(claims.tenant_id)
    .bind(claims.sub)
    .bind("POLICY_UPDATED")
    .bind("policy")
    .bind(policy.id)
    .execute(&state.db_pool)
    .await?;

    Ok((StatusCode::OK, Json(policy.into())))
}

pub async fn delete_policy(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path(id): Path<PolicyId>,
) -> Result<(StatusCode, Json<Value>)> {
    let auth_header = headers
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| Error::Authentication("Missing authorization header".to_string()))?;

    let token = auth_header.strip_prefix("Bearer ")
        .ok_or_else(|| Error::Authentication("Invalid authorization header".to_string()))?;

    let claims = state.jwt_manager.verify_access_token(token)?;

    let result = sqlx::query("DELETE FROM policies WHERE id = $1 AND tenant_id = $2")
        .bind(id)
        .bind(claims.tenant_id)
        .execute(&state.db_pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(Error::NotFound("Policy not found".to_string()));
    }

    // Log audit
    sqlx::query(
        "INSERT INTO audit_logs (tenant_id, user_id, action, resource_type, resource_id)
         VALUES ($1, $2, $3, $4, $5)"
    )
    .bind(claims.tenant_id)
    .bind(claims.sub)
    .bind("POLICY_DELETED")
    .bind("policy")
    .bind(id)
    .execute(&state.db_pool)
    .await?;

    Ok((
        StatusCode::OK,
        Json(json!({
            "message": "Policy deleted successfully"
        })),
    ))
}

#[derive(Debug, Deserialize)]
pub struct CheckPolicyRequest {
    pub user_id: Option<Uuid>,
    pub device_id: Uuid,
    pub target_device_id: Uuid,
    pub action: String, // "connect", "file_transfer", "clipboard", etc.
    pub ip_address: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CheckPolicyResponse {
    pub allowed: bool,
    pub reason: Option<String>,
    pub policy_id: Option<PolicyId>,
}

pub async fn check_policy(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(payload): Json<CheckPolicyRequest>,
) -> Result<(StatusCode, Json<CheckPolicyResponse>)> {
    let auth_header = headers
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| Error::Authentication("Missing authorization header".to_string()))?;

    let token = auth_header.strip_prefix("Bearer ")
        .ok_or_else(|| Error::Authentication("Invalid authorization header".to_string()))?;

    let claims = state.jwt_manager.verify_access_token(token)?;

    // Get device groups
    let device_groups: Vec<Uuid> = sqlx::query_scalar(
        "SELECT group_id FROM device_groups WHERE device_id = $1"
    )
    .bind(payload.device_id)
    .fetch_all(&state.db_pool)
    .await?;

    if device_groups.is_empty() {
        // No groups = no policies = allow by default (or you can deny by default)
        return Ok((
            StatusCode::OK,
            Json(CheckPolicyResponse {
                allowed: true,
                reason: Some("No policies assigned".to_string()),
                policy_id: None,
            }),
        ));
    }

    // Get policies for these groups
    let policies = sqlx::query_as::<_, Policy>(
        "SELECT DISTINCT p.* FROM policies p
         JOIN group_policies gp ON p.id = gp.policy_id
         WHERE gp.group_id = ANY($1) AND p.is_active = true AND p.tenant_id = $2"
    )
    .bind(&device_groups)
    .bind(claims.tenant_id)
    .fetch_all(&state.db_pool)
    .await?;

    // Check each policy
    for policy in policies {
        let rules = &policy.rules;

        // Check action-specific rules
        match payload.action.as_str() {
            "clipboard" => {
                if !rules.allow_clipboard {
                    return Ok((
                        StatusCode::OK,
                        Json(CheckPolicyResponse {
                            allowed: false,
                            reason: Some("Clipboard not allowed by policy".to_string()),
                            policy_id: Some(policy.id),
                        }),
                    ));
                }
            }
            "file_transfer" => {
                if !rules.allow_file_transfer {
                    return Ok((
                        StatusCode::OK,
                        Json(CheckPolicyResponse {
                            allowed: false,
                            reason: Some("File transfer not allowed by policy".to_string()),
                            policy_id: Some(policy.id),
                        }),
                    ));
                }
            }
            "audio" => {
                if !rules.allow_audio {
                    return Ok((
                        StatusCode::OK,
                        Json(CheckPolicyResponse {
                            allowed: false,
                            reason: Some("Audio not allowed by policy".to_string()),
                            policy_id: Some(policy.id),
                        }),
                    ));
                }
            }
            _ => {}
        }

        // Check IP whitelist/blacklist
        if let Some(ip) = &payload.ip_address {
            if let Some(blacklist) = &rules.ip_blacklist {
                if blacklist.contains(ip) {
                    return Ok((
                        StatusCode::OK,
                        Json(CheckPolicyResponse {
                            allowed: false,
                            reason: Some("IP address is blacklisted".to_string()),
                            policy_id: Some(policy.id),
                        }),
                    ));
                }
            }

            if let Some(whitelist) = &rules.ip_whitelist {
                if !whitelist.is_empty() && !whitelist.contains(ip) {
                    return Ok((
                        StatusCode::OK,
                        Json(CheckPolicyResponse {
                            allowed: false,
                            reason: Some("IP address not in whitelist".to_string()),
                            policy_id: Some(policy.id),
                        }),
                    ));
                }
            }
        }

        // Check time restrictions (TODO: implement time checks)
        // Check day restrictions (TODO: implement day checks)
    }

    Ok((
        StatusCode::OK,
        Json(CheckPolicyResponse {
            allowed: true,
            reason: None,
            policy_id: None,
        }),
    ))
}

pub async fn get_policy_groups(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path(id): Path<PolicyId>,
) -> Result<(StatusCode, Json<Vec<Uuid>>)> {
    let auth_header = headers
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| Error::Authentication("Missing authorization header".to_string()))?;

    let token = auth_header.strip_prefix("Bearer ")
        .ok_or_else(|| Error::Authentication("Invalid authorization header".to_string()))?;

    let claims = state.jwt_manager.verify_access_token(token)?;

    let group_ids: Vec<Uuid> = sqlx::query_scalar(
        "SELECT group_id FROM group_policies WHERE policy_id = $1"
    )
    .bind(id)
    .fetch_all(&state.db_pool)
    .await?;

    Ok((StatusCode::OK, Json(group_ids)))
}

pub async fn assign_policy_to_group(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path((id, group_id)): Path<(PolicyId, Uuid)>,
) -> Result<(StatusCode, Json<Value>)> {
    let auth_header = headers
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| Error::Authentication("Missing authorization header".to_string()))?;

    let token = auth_header.strip_prefix("Bearer ")
        .ok_or_else(|| Error::Authentication("Invalid authorization header".to_string()))?;

    let claims = state.jwt_manager.verify_access_token(token)?;

    sqlx::query(
        "INSERT INTO group_policies (group_id, policy_id) VALUES ($1, $2)"
    )
    .bind(group_id)
    .bind(id)
    .execute(&state.db_pool)
    .await
    .map_err(|e| {
        if e.to_string().contains("duplicate key") {
            Error::Validation("Policy already assigned to group".to_string())
        } else {
            Error::Database(e)
        }
    })?;

    Ok((
        StatusCode::CREATED,
        Json(json!({
            "message": "Policy assigned to group"
        })),
    ))
}

pub async fn unassign_policy_from_group(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path((id, group_id)): Path<(PolicyId, Uuid)>,
) -> Result<(StatusCode, Json<Value>)> {
    let auth_header = headers
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| Error::Authentication("Missing authorization header".to_string()))?;

    let token = auth_header.strip_prefix("Bearer ")
        .ok_or_else(|| Error::Authentication("Invalid authorization header".to_string()))?;

    let claims = state.jwt_manager.verify_access_token(token)?;

    let result = sqlx::query(
        "DELETE FROM group_policies WHERE group_id = $1 AND policy_id = $2"
    )
    .bind(group_id)
    .bind(id)
    .execute(&state.db_pool)
    .await?;

    if result.rows_affected() == 0 {
        return Err(Error::NotFound("Policy not assigned to group".to_string()));
    }

    Ok((
        StatusCode::OK,
        Json(json!({
            "message": "Policy unassigned from group"
        })),
    ))
}
