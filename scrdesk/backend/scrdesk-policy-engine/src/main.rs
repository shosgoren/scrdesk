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
                .unwrap_or_else(|_| "scrdesk_policy_engine=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Starting ScrDesk Policy Engine...");

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
        // Policy CRUD
        .route("/api/v1/policies", post(handlers::policies::create_policy))
        .route("/api/v1/policies", get(handlers::policies::list_policies))
        .route("/api/v1/policies/:id", get(handlers::policies::get_policy))
        .route("/api/v1/policies/:id", put(handlers::policies::update_policy))
        .route("/api/v1/policies/:id", delete(handlers::policies::delete_policy))
        // Policy enforcement
        .route("/api/v1/policies/check", post(handlers::policies::check_policy))
        .route("/api/v1/policies/:id/groups", get(handlers::policies::get_policy_groups))
        .route("/api/v1/policies/:id/groups/:group_id", post(handlers::policies::assign_policy_to_group))
        .route("/api/v1/policies/:id/groups/:group_id", delete(handlers::policies::unassign_policy_from_group))
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
