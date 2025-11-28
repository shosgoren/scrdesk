use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::{PolicyId, TenantId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyRules {
    pub allow_clipboard: bool,
    pub allow_file_transfer: bool,
    pub allow_audio: bool,
    pub allow_recording: bool,
    pub require_approval: bool,
    pub allowed_hours: Option<Vec<String>>, // ["09:00-17:00"]
    pub allowed_days: Option<Vec<u8>>, // [1,2,3,4,5] (Monday-Friday)
    pub ip_whitelist: Option<Vec<String>>,
    pub ip_blacklist: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Policy {
    pub id: PolicyId,
    pub tenant_id: TenantId,
    pub name: String,
    pub description: Option<String>,
    #[sqlx(json)]
    pub rules: PolicyRules,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct PolicyResponse {
    pub id: PolicyId,
    pub name: String,
    pub description: Option<String>,
    pub rules: PolicyRules,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<Policy> for PolicyResponse {
    fn from(policy: Policy) -> Self {
        Self {
            id: policy.id,
            name: policy.name,
            description: policy.description,
            rules: policy.rules,
            is_active: policy.is_active,
            created_at: policy.created_at,
            updated_at: policy.updated_at,
        }
    }
}
