use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;

use super::{DeviceId, TenantId, UserId};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, sqlx::Type, PartialEq, Eq)]
#[sqlx(type_name = "device_platform", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum DevicePlatform {
    Windows,
    MacOS,
    Linux,
    Android,
    iOS,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, sqlx::Type, PartialEq, Eq)]
#[sqlx(type_name = "device_status", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum DeviceStatus {
    Online,
    Offline,
    Busy,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Device {
    pub id: DeviceId,
    pub tenant_id: TenantId,
    pub owner_id: Option<UserId>,
    pub device_id: String, // Custom ID shown to users
    pub device_name: String,
    pub platform: DevicePlatform,
    pub os_version: String,
    pub client_version: String,
    pub status: DeviceStatus,
    pub last_seen_at: Option<DateTime<Utc>>,
    pub ip_address: Option<String>,
    pub public_key: String,
    pub is_approved: bool,
    pub metadata: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Validate, Deserialize)]
pub struct RegisterDeviceRequest {
    #[validate(length(min = 1, max = 50))]
    pub device_id: String,

    #[validate(length(min = 1, max = 100))]
    pub device_name: String,

    pub platform: DevicePlatform,

    #[validate(length(min = 1, max = 50))]
    pub os_version: String,

    #[validate(length(min = 1, max = 50))]
    pub client_version: String,

    pub public_key: String,
}

#[derive(Debug, Validate, Deserialize)]
pub struct UpdateDeviceRequest {
    #[validate(length(min = 1, max = 100))]
    pub device_name: Option<String>,

    pub status: Option<DeviceStatus>,
    pub is_approved: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct DeviceResponse {
    pub id: DeviceId,
    pub device_id: String,
    pub device_name: String,
    pub platform: DevicePlatform,
    pub os_version: String,
    pub client_version: String,
    pub status: DeviceStatus,
    pub last_seen_at: Option<DateTime<Utc>>,
    pub is_approved: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<Device> for DeviceResponse {
    fn from(device: Device) -> Self {
        Self {
            id: device.id,
            device_id: device.device_id,
            device_name: device.device_name,
            platform: device.platform,
            os_version: device.os_version,
            client_version: device.client_version,
            status: device.status,
            last_seen_at: device.last_seen_at,
            is_approved: device.is_approved,
            created_at: device.created_at,
            updated_at: device.updated_at,
        }
    }
}
