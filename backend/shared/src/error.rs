use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Authentication error: {0}")]
    Authentication(String),

    #[error("Authorization error: {0}")]
    Authorization(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JWT error: {0}")]
    Jwt(#[from] jsonwebtoken::errors::Error),

    #[error("Tenant error: {0}")]
    Tenant(String),

    #[error("Device limit exceeded")]
    DeviceLimitExceeded,

    #[error("Session error: {0}")]
    Session(String),

    #[error("Policy violation: {0}")]
    PolicyViolation(String),

    #[error("Billing error: {0}")]
    Billing(String),

    #[error("Internal server error: {0}")]
    Internal(String),

    #[error("External service error: {0}")]
    ExternalService(String),
}

impl Error {
    pub fn status_code(&self) -> u16 {
        match self {
            Error::Database(_) => 500,
            Error::Authentication(_) => 401,
            Error::Authorization(_) => 403,
            Error::NotFound(_) => 404,
            Error::Validation(_) => 400,
            Error::Config(_) => 500,
            Error::Io(_) => 500,
            Error::Jwt(_) => 401,
            Error::Tenant(_) => 403,
            Error::DeviceLimitExceeded => 429,
            Error::Session(_) => 400,
            Error::PolicyViolation(_) => 403,
            Error::Billing(_) => 402,
            Error::Internal(_) => 500,
            Error::ExternalService(_) => 502,
        }
    }

    pub fn error_code(&self) -> &'static str {
        match self {
            Error::Database(_) => "DATABASE_ERROR",
            Error::Authentication(_) => "AUTHENTICATION_ERROR",
            Error::Authorization(_) => "AUTHORIZATION_ERROR",
            Error::NotFound(_) => "NOT_FOUND",
            Error::Validation(_) => "VALIDATION_ERROR",
            Error::Config(_) => "CONFIG_ERROR",
            Error::Io(_) => "IO_ERROR",
            Error::Jwt(_) => "JWT_ERROR",
            Error::Tenant(_) => "TENANT_ERROR",
            Error::DeviceLimitExceeded => "DEVICE_LIMIT_EXCEEDED",
            Error::Session(_) => "SESSION_ERROR",
            Error::PolicyViolation(_) => "POLICY_VIOLATION",
            Error::Billing(_) => "BILLING_ERROR",
            Error::Internal(_) => "INTERNAL_ERROR",
            Error::ExternalService(_) => "EXTERNAL_SERVICE_ERROR",
        }
    }
}
