use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};

use crate::config::DatabaseConfig;
use crate::error::Result;

pub type DbPool = Pool<Postgres>;

pub async fn create_pool(config: &DatabaseConfig) -> Result<DbPool> {
    let pool = PgPoolOptions::new()
        .max_connections(config.max_connections)
        .min_connections(config.min_connections)
        .connect(&config.url)
        .await?;

    Ok(pool)
}

pub async fn run_migrations(_pool: &DbPool) -> Result<()> {
    // TODO: Add migrations directory
    // sqlx::migrate!("./migrations")
    //     .run(pool)
    //     .await?;
    Ok(())
}
