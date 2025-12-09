use axum::{
    extract::{Path, Query, State},
    http::{HeaderMap, StatusCode},
    Json,
};
use chrono::{DateTime, Utc};
use scrdesk_shared::{
    error::{Error, Result},
    models::{
        audit::{AuditLog, AuditLogResponse},
        PaginationParams, PaginatedResponse,
    },
};
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;

use crate::AppState;

#[derive(Debug, Deserialize)]
pub struct AuditLogQuery {
    #[serde(flatten)]
    pub pagination: PaginationParams,

    pub user_id: Option<Uuid>,
    pub action: Option<String>,
    pub resource_type: Option<String>,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
}

pub async fn list_audit_logs(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Query(query): Query<AuditLogQuery>,
) -> Result<(StatusCode, Json<PaginatedResponse<AuditLogResponse>>)> {
    let auth_header = headers
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| Error::Authentication("Missing authorization header".to_string()))?;

    let token = auth_header.strip_prefix("Bearer ")
        .ok_or_else(|| Error::Authentication("Invalid authorization header".to_string()))?;

    let claims = state.jwt_manager.verify_access_token(token)?;

    // Build dynamic query
    let mut where_clauses = vec!["tenant_id = $1".to_string()];
    let mut bind_count = 2;

    if query.user_id.is_some() {
        where_clauses.push(format!("user_id = ${}", bind_count));
        bind_count += 1;
    }

    if query.action.is_some() {
        where_clauses.push(format!("action = ${}", bind_count));
        bind_count += 1;
    }

    if query.resource_type.is_some() {
        where_clauses.push(format!("resource_type = ${}", bind_count));
        bind_count += 1;
    }

    if query.start_date.is_some() {
        where_clauses.push(format!("created_at >= ${}", bind_count));
        bind_count += 1;
    }

    if query.end_date.is_some() {
        where_clauses.push(format!("created_at <= ${}", bind_count));
        bind_count += 1;
    }

    let where_clause = where_clauses.join(" AND ");

    // Count total
    let count_query = format!("SELECT COUNT(*) FROM audit_logs WHERE {}", where_clause);
    let mut count_builder = sqlx::query_scalar::<_, i64>(&count_query)
        .bind(claims.tenant_id);

    if let Some(user_id) = query.user_id {
        count_builder = count_builder.bind(user_id);
    }
    if let Some(action) = &query.action {
        count_builder = count_builder.bind(action);
    }
    if let Some(resource_type) = &query.resource_type {
        count_builder = count_builder.bind(resource_type);
    }
    if let Some(start_date) = query.start_date {
        count_builder = count_builder.bind(start_date);
    }
    if let Some(end_date) = query.end_date {
        count_builder = count_builder.bind(end_date);
    }

    let total = count_builder.fetch_one(&state.db_pool).await?;

    // Fetch logs
    let logs_query = format!(
        "SELECT * FROM audit_logs WHERE {} ORDER BY created_at DESC LIMIT ${} OFFSET ${}",
        where_clause,
        bind_count,
        bind_count + 1
    );

    let mut logs_builder = sqlx::query_as::<_, AuditLog>(&logs_query)
        .bind(claims.tenant_id);

    if let Some(user_id) = query.user_id {
        logs_builder = logs_builder.bind(user_id);
    }
    if let Some(action) = &query.action {
        logs_builder = logs_builder.bind(action);
    }
    if let Some(resource_type) = &query.resource_type {
        logs_builder = logs_builder.bind(resource_type);
    }
    if let Some(start_date) = query.start_date {
        logs_builder = logs_builder.bind(start_date);
    }
    if let Some(end_date) = query.end_date {
        logs_builder = logs_builder.bind(end_date);
    }

    let logs = logs_builder
        .bind(query.pagination.limit() as i32)
        .bind(query.pagination.offset() as i32)
        .fetch_all(&state.db_pool)
        .await?;

    let log_responses: Vec<_> = logs.into_iter().map(|l| l.into()).collect();

    Ok((
        StatusCode::OK,
        Json(PaginatedResponse::new(log_responses, total, query.pagination)),
    ))
}

pub async fn get_audit_log(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, Json<AuditLogResponse>)> {
    let auth_header = headers
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| Error::Authentication("Missing authorization header".to_string()))?;

    let token = auth_header.strip_prefix("Bearer ")
        .ok_or_else(|| Error::Authentication("Invalid authorization header".to_string()))?;

    let claims = state.jwt_manager.verify_access_token(token)?;

    let log = sqlx::query_as::<_, AuditLog>(
        "SELECT * FROM audit_logs WHERE id = $1 AND tenant_id = $2"
    )
    .bind(id)
    .bind(claims.tenant_id)
    .fetch_optional(&state.db_pool)
    .await?
    .ok_or_else(|| Error::NotFound("Audit log not found".to_string()))?;

    Ok((StatusCode::OK, Json(log.into())))
}

pub async fn export_audit_logs(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Query(query): Query<AuditLogQuery>,
) -> Result<(StatusCode, Json<Vec<AuditLogResponse>>)> {
    // Similar to list but without pagination, for CSV/JSON export
    let auth_header = headers
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| Error::Authentication("Missing authorization header".to_string()))?;

    let token = auth_header.strip_prefix("Bearer ")
        .ok_or_else(|| Error::Authentication("Invalid authorization header".to_string()))?;

    let claims = state.jwt_manager.verify_access_token(token)?;

    let logs = sqlx::query_as::<_, AuditLog>(
        "SELECT * FROM audit_logs WHERE tenant_id = $1 ORDER BY created_at DESC LIMIT 10000"
    )
    .bind(claims.tenant_id)
    .fetch_all(&state.db_pool)
    .await?;

    let log_responses: Vec<_> = logs.into_iter().map(|l| l.into()).collect();

    Ok((StatusCode::OK, Json(log_responses)))
}
