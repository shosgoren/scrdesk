use axum::{routing::get, Router};
use scrdesk_shared::config::Config;
use std::sync::Arc;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod handlers;
mod relay;

pub struct AppState {
    pub config: Config,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| "scrdesk_relay=info".into()))
        .with(tracing_subscriber::fmt::layer()).init();

    tracing::info!("Starting ScrDesk Relay Cluster...");

    dotenv::dotenv().ok();
    let config = Config::from_env()?;
    let state = Arc::new(AppState { config: config.clone() });

    // Management API
    let app = Router::new()
        .route("/health", get(handlers::health::health_check))
        .route("/api/v1/relay/status", get(handlers::relay::get_relay_status))
        .with_state(state.clone());

    let mgmt_addr = format!("{}:21116", config.server.host);
    tracing::info!("Management API listening on {}", mgmt_addr);

    // Start relay server in background
    let relay_config = config.clone();
    tokio::spawn(async move {
        if let Err(e) = relay::start_relay_server(relay_config).await {
            tracing::error!("Relay server error: {}", e);
        }
    });

    let listener = tokio::net::TcpListener::bind(&mgmt_addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
