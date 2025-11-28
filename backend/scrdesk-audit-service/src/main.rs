use axum::{routing::get, Router};
use scrdesk_shared::{config::Config, database};
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod handlers;

pub struct AppState {
    pub db_pool: sqlx::PgPool,
    pub redis_client: redis::Client,
    pub jwt_manager: Arc<scrdesk_shared::auth::JwtManager>,
    pub config: Config,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "scrdesk_audit_service=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Starting ScrDesk Audit Service...");

    dotenv::dotenv().ok();
    let config = Config::from_env()?;

    let db_pool = database::create_pool(&config.database).await?;
    let redis_client = redis::Client::open(config.redis.url.as_str())?;

    let jwt_manager = Arc::new(scrdesk_shared::auth::JwtManager::new(
        &config.jwt.secret,
        config.jwt.access_token_expiry,
        config.jwt.refresh_token_expiry,
    ));

    let state = Arc::new(AppState {
        db_pool,
        redis_client,
        jwt_manager,
        config: config.clone(),
    });

    let app = Router::new()
        .route("/health", get(handlers::health::health_check))
        .route("/api/v1/audit-logs", get(handlers::audit::list_audit_logs))
        .route("/api/v1/audit-logs/:id", get(handlers::audit::get_audit_log))
        .route("/api/v1/audit-logs/export", get(handlers::audit::export_audit_logs))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
        .with_state(state);

    let addr = format!("{}:{}", config.server.host, config.server.port);
    tracing::info!("Listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
