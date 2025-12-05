use axum::{
    routing::{get, post},
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
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "scrdesk_auth_service=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Starting ScrDesk Auth Service...");

    // Load configuration
    dotenv::dotenv().ok();
    let config = Config::from_env()?;

    tracing::info!("Connecting to database...");
    let db_pool = database::create_pool(&config.database).await?;

    tracing::info!("Running database migrations...");
    database::run_migrations(&db_pool).await?;

    tracing::info!("Connecting to Redis...");
    let redis_client = redis::Client::open(config.redis.url.as_str())?;

    // Test Redis connection
    let mut conn = redis_client.get_multiplexed_async_connection().await?;
    redis::cmd("PING")
        .query_async::<_, String>(&mut conn)
        .await?;
    tracing::info!("Redis connected successfully");

    // Create JWT manager
    let jwt_manager = Arc::new(scrdesk_shared::auth::JwtManager::new(
        &config.jwt.secret,
        config.jwt.access_token_expiry,
        config.jwt.refresh_token_expiry,
    ));

    // Create shared state
    let state = Arc::new(AppState {
        db_pool,
        redis_client,
        jwt_manager,
        config: config.clone(),
    });

    // Build router
    let app = Router::new()
        .route("/health", get(handlers::health::health_check))
        .route("/api/v1/auth/register", post(handlers::auth::register))
        .route("/api/v1/auth/login", post(handlers::auth::login))
        .route("/api/v1/auth/refresh", post(handlers::auth::refresh_token))
        .route("/api/v1/auth/logout", post(handlers::auth::logout))
        .route("/api/v1/auth/me", get(handlers::auth::get_current_user))
        .route("/api/v1/auth/2fa/enable", post(handlers::two_factor::enable_2fa))
        .route("/api/v1/auth/2fa/verify", post(handlers::two_factor::verify_2fa))
        .route("/api/v1/auth/2fa/disable", post(handlers::two_factor::disable_2fa))
        .route("/api/v1/auth/password/change", post(handlers::auth::change_password))
        .route("/api/v1/auth/password/reset", post(handlers::auth::request_password_reset))
        .route("/api/v1/auth/password/reset/confirm", post(handlers::auth::confirm_password_reset))
        .route("/api/v1/auth/oauth/google", get(handlers::oauth::google_auth_url))
        .route("/api/v1/auth/oauth/google/callback", get(handlers::oauth::google_callback))
        .route("/api/v1/auth/oauth/apple", get(handlers::oauth::apple_auth_url))
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
