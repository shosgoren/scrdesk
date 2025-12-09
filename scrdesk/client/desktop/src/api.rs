use anyhow::{Context, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Clone)]
pub struct ApiClient {
    base_url: String,
    client: Client,
    token: Arc<Mutex<Option<String>>>,
}

#[derive(Debug, Serialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub totp_code: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct LoginResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub user: UserInfo,
}

#[derive(Debug, Deserialize, Clone)]
pub struct UserInfo {
    pub id: String,
    pub email: String,
    pub tenant_id: String,
}

#[derive(Debug, Serialize)]
pub struct RegisterDeviceRequest {
    pub device_name: String,
    pub os_type: String,
    pub os_version: String,
    pub hostname: String,
}

#[derive(Debug, Deserialize)]
pub struct RegisterDeviceResponse {
    pub device_id: String,
    pub device_key: String,
}

#[derive(Debug, Deserialize)]
pub struct Device {
    pub id: String,
    pub device_name: String,
    pub device_key: String,
    pub status: String,
    pub last_seen: Option<String>,
    pub approved: bool,
}

#[derive(Debug, Deserialize)]
pub struct DeviceListResponse {
    pub devices: Vec<Device>,
}

impl ApiClient {
    pub fn new(base_url: String) -> Self {
        Self {
            base_url,
            client: Client::builder()
                .timeout(std::time::Duration::from_secs(10))
                .build()
                .expect("Failed to build HTTP client"),
            token: Arc::new(Mutex::new(None)),
        }
    }

    pub async fn login(&self, email: String, password: String, totp_code: Option<String>) -> Result<LoginResponse> {
        let url = format!("{}/api/v1/auth/login", self.base_url);
        let request = LoginRequest {
            email,
            password,
            totp_code,
        };

        let response = self.client
            .post(&url)
            .json(&request)
            .send()
            .await
            .context("Failed to send login request")?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            anyhow::bail!("Login failed: {} - {}", status, body);
        }

        let login_response: LoginResponse = response.json().await
            .context("Failed to parse login response")?;

        // Store token
        *self.token.lock().await = Some(login_response.access_token.clone());

        Ok(login_response)
    }

    pub async fn register_device(&self, request: RegisterDeviceRequest) -> Result<RegisterDeviceResponse> {
        let url = format!("{}/api/v1/devices", self.base_url);

        let token = self.token.lock().await.clone()
            .context("Not authenticated")?;

        let response = self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", token))
            .json(&request)
            .send()
            .await
            .context("Failed to register device")?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            anyhow::bail!("Device registration failed: {} - {}", status, body);
        }

        let device_response: RegisterDeviceResponse = response.json().await
            .context("Failed to parse device registration response")?;

        Ok(device_response)
    }

    pub async fn list_devices(&self) -> Result<Vec<Device>> {
        let url = format!("{}/api/v1/devices", self.base_url);

        let token = self.token.lock().await.clone()
            .context("Not authenticated")?;

        let response = self.client
            .get(&url)
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await
            .context("Failed to list devices")?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            anyhow::bail!("Device list failed: {} - {}", status, body);
        }

        let device_list: DeviceListResponse = response.json().await
            .context("Failed to parse device list response")?;

        Ok(device_list.devices)
    }

    pub async fn send_heartbeat(&self, device_id: &str) -> Result<()> {
        let url = format!("{}/api/v1/devices/{}/heartbeat", self.base_url, device_id);

        let token = self.token.lock().await.clone()
            .context("Not authenticated")?;

        let response = self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await
            .context("Failed to send heartbeat")?;

        if !response.status().is_success() {
            let status = response.status();
            anyhow::bail!("Heartbeat failed: {}", status);
        }

        Ok(())
    }

    pub fn set_token(&self, token: String) {
        let token_clone = Arc::clone(&self.token);
        tokio::spawn(async move {
            *token_clone.lock().await = Some(token);
        });
    }

    pub async fn is_authenticated(&self) -> bool {
        self.token.lock().await.is_some()
    }
}
