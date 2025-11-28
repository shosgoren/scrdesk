use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;

use super::{PlanType, TenantId};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Tenant {
    pub id: TenantId,
    pub name: String,
    pub slug: String,
    pub plan: PlanType,
    pub is_active: bool,
    pub device_limit: Option<i32>,
    pub max_concurrent_sessions: Option<i32>,
    pub custom_domain: Option<String>,
    pub relay_servers: Vec<String>, // JSON array in DB
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Validate, Deserialize)]
pub struct CreateTenantRequest {
    #[validate(length(min = 1, max = 100))]
    pub name: String,

    #[validate(length(min = 1, max = 50), regex = "^[a-z0-9-]+$")]
    pub slug: String,

    pub plan: PlanType,
    pub custom_domain: Option<String>,
}

#[derive(Debug, Validate, Deserialize)]
pub struct UpdateTenantRequest {
    #[validate(length(min = 1, max = 100))]
    pub name: Option<String>,

    pub plan: Option<PlanType>,
    pub is_active: Option<bool>,
    pub device_limit: Option<i32>,
    pub max_concurrent_sessions: Option<i32>,
    pub custom_domain: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct TenantResponse {
    pub id: TenantId,
    pub name: String,
    pub slug: String,
    pub plan: PlanType,
    pub is_active: bool,
    pub device_limit: Option<i32>,
    pub max_concurrent_sessions: Option<i32>,
    pub custom_domain: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<Tenant> for TenantResponse {
    fn from(tenant: Tenant) -> Self {
        Self {
            id: tenant.id,
            name: tenant.name,
            slug: tenant.slug,
            plan: tenant.plan,
            is_active: tenant.is_active,
            device_limit: tenant.device_limit,
            max_concurrent_sessions: tenant.max_concurrent_sessions,
            custom_domain: tenant.custom_domain,
            created_at: tenant.created_at,
            updated_at: tenant.updated_at,
        }
    }
}
