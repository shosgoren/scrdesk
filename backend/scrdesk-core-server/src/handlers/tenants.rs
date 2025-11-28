use axum::{extract::{Path, State}, http::StatusCode, Json};
use scrdesk_shared::{
    error::{Error, Result},
    models::tenant::{CreateTenantRequest, TenantResponse},
};
use std::sync::Arc;
use validator::Validate;

use crate::AppState;

pub async fn create_tenant(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateTenantRequest>,
) -> Result<(StatusCode, Json<TenantResponse>)> {
    payload.validate().map_err(|e| Error::Validation(e.to_string()))?;

    let tenant = sqlx::query_as::<_, scrdesk_shared::models::tenant::Tenant>(
        "INSERT INTO tenants (name, slug, plan) VALUES ($1, $2, $3) RETURNING *"
    )
    .bind(&payload.name)
    .bind(&payload.slug)
    .bind(&payload.plan)
    .fetch_one(&state.db_pool)
    .await?;

    Ok((StatusCode::CREATED, Json(tenant.into())))
}

pub async fn get_tenant(
    State(state): State<Arc<AppState>>,
    Path(id): Path<uuid::Uuid>,
) -> Result<(StatusCode, Json<TenantResponse>)> {
    let tenant = sqlx::query_as::<_, scrdesk_shared::models::tenant::Tenant>(
        "SELECT * FROM tenants WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(&state.db_pool)
    .await?
    .ok_or_else(|| Error::NotFound("Tenant not found".to_string()))?;

    Ok((StatusCode::OK, Json(tenant.into())))
}
