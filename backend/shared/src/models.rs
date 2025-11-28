pub mod tenant;
pub mod user;
pub mod device;
pub mod session;
pub mod policy;
pub mod audit;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

// Common types
pub type TenantId = Uuid;
pub type UserId = Uuid;
pub type DeviceId = Uuid;
pub type SessionId = Uuid;
pub type PolicyId = Uuid;
pub type GroupId = Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationParams {
    #[serde(default = "default_page")]
    pub page: u32,
    #[serde(default = "default_per_page")]
    pub per_page: u32,
}

fn default_page() -> u32 {
    1
}

fn default_per_page() -> u32 {
    20
}

impl PaginationParams {
    pub fn offset(&self) -> u32 {
        (self.page - 1) * self.per_page
    }

    pub fn limit(&self) -> u32 {
        self.per_page
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub total: i64,
    pub page: u32,
    pub per_page: u32,
    pub total_pages: u32,
}

impl<T> PaginatedResponse<T> {
    pub fn new(data: Vec<T>, total: i64, pagination: PaginationParams) -> Self {
        let total_pages = ((total as f64) / (pagination.per_page as f64)).ceil() as u32;
        Self {
            data,
            total,
            page: pagination.page,
            per_page: pagination.per_page,
            total_pages,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, sqlx::Type, PartialEq, Eq)]
#[sqlx(type_name = "user_role", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum UserRole {
    SuperAdmin,
    OrgAdmin,
    Admin,
    User,
    ReadOnly,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, sqlx::Type, PartialEq, Eq)]
#[sqlx(type_name = "plan_type", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum PlanType {
    Free,
    Pro,
    Enterprise,
}

impl PlanType {
    pub fn device_limit(&self) -> Option<u32> {
        match self {
            PlanType::Free => Some(20),
            PlanType::Pro => Some(500),
            PlanType::Enterprise => None, // Unlimited
        }
    }

    pub fn max_concurrent_sessions(&self) -> Option<u32> {
        match self {
            PlanType::Free => Some(3),
            PlanType::Pro => Some(50),
            PlanType::Enterprise => None, // Unlimited
        }
    }
}
