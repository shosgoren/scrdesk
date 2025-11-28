use axum::{routing::{get, post}, Router};
use scrdesk_shared::config::Config;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| "scrdesk_billing=info".into()))
        .with(tracing_subscriber::fmt::layer()).init();
    tracing::info!("Starting ScrDesk Billing Service...");
    
    dotenv::dotenv().ok();
    let config = Config::from_env()?;
    
    let app = Router::new()
        .route("/api/v1/billing/subscriptions", get(|| async { "List subscriptions" }))
        .route("/api/v1/billing/webhook", post(|| async { "Stripe webhook" }));
    
    let addr = format!("{}:{}", config.server.host, config.server.port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
