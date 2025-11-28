use axum::{routing::post, Router};
use scrdesk_shared::{config::Config, database};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| "scrdesk_notification=info".into()))
        .with(tracing_subscriber::fmt::layer()).init();
    tracing::info!("Starting ScrDesk Notification Service...");
    
    dotenv::dotenv().ok();
    let config = Config::from_env()?;
    
    let app = Router::new()
        .route("/api/v1/notifications/send-email", post(|| async { "Email sent" }));
    
    let addr = format!("{}:{}", config.server.host, config.server.port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
