use axum::{
    routing::{get, post, put, delete},
    Router,
};
use scrdesk_shared::{config::Config, database};
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod handlers;
mod services;
mod repositories;

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
                .unwrap_or_else(|_| "scrdesk_device_manager=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Starting ScrDesk Device Manager...");

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
        // Device management endpoints
        .route("/api/v1/devices", post(handlers::devices::register_device))
        .route("/api/v1/devices", get(handlers::devices::list_devices))
        .route("/api/v1/devices/:id", get(handlers::devices::get_device))
        .route("/api/v1/devices/:id", put(handlers::devices::update_device))
        .route("/api/v1/devices/:id", delete(handlers::devices::delete_device))
        .route("/api/v1/devices/:id/approve", post(handlers::devices::approve_device))
        .route("/api/v1/devices/:id/revoke", post(handlers::devices::revoke_device))
        .route("/api/v1/devices/:id/heartbeat", post(handlers::devices::device_heartbeat))
        .route("/api/v1/devices/:id/status", put(handlers::devices::update_device_status))
        // Device groups
        .route("/api/v1/devices/:id/groups", get(handlers::devices::get_device_groups))
        .route("/api/v1/devices/:id/groups/:group_id", post(handlers::devices::add_device_to_group))
        .route("/api/v1/devices/:id/groups/:group_id", delete(handlers::devices::remove_device_from_group))
        // Device connection
        .route("/api/v1/devices/:id/connect", post(handlers::devices::request_connection))
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
