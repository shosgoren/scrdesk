use axum::{routing::get, Router, Json};
use serde_json::json;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| "scrdesk_update=info".into()))
        .with(tracing_subscriber::fmt::layer()).init();
    tracing::info!("Starting ScrDesk Update Server...");
    
    dotenv::dotenv().ok();
    
    let app = Router::new()
        .route("/api/v1/updates/latest", get(|| async {
            Json(json!({"version": "1.0.0", "download_url": "https://github.com/shosgoren/scrdesk/releases/latest"}))
        }));
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    axum::serve(listener, app).await?;
    Ok(())
}
