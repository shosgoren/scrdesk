use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::{DeviceId, SessionId, TenantId, UserId};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Session {
    pub id: SessionId,
    pub tenant_id: TenantId,
    pub initiator_device_id: DeviceId,
    pub target_device_id: DeviceId,
    pub initiator_user_id: Option<UserId>,
    pub started_at: DateTime<Utc>,
    pub ended_at: Option<DateTime<Utc>>,
    pub duration_seconds: Option<i32>,
    pub relay_server: String,
    pub recording_url: Option<String>,
    pub is_recorded: bool,
    pub metadata: serde_json::Value,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct SessionResponse {
    pub id: SessionId,
    pub initiator_device_id: DeviceId,
    pub target_device_id: DeviceId,
    pub started_at: DateTime<Utc>,
    pub ended_at: Option<DateTime<Utc>>,
    pub duration_seconds: Option<i32>,
    pub relay_server: String,
    pub is_recorded: bool,
    pub created_at: DateTime<Utc>,
}

impl From<Session> for SessionResponse {
    fn from(session: Session) -> Self {
        Self {
            id: session.id,
            initiator_device_id: session.initiator_device_id,
            target_device_id: session.target_device_id,
            started_at: session.started_at,
            ended_at: session.ended_at,
            duration_seconds: session.duration_seconds,
            relay_server: session.relay_server,
            is_recorded: session.is_recorded,
            created_at: session.created_at,
        }
    }
}
