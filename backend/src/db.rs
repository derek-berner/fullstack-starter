use sqlx::postgres::PgPoolOptions;
use crate::error::Error;

pub async fn create_pool(database_url: &str) -> Result<sqlx::PgPool, Error> {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
        .map_err(Error::Database)
}

pub async fn run_migrations(pool: &sqlx::PgPool) -> Result<(), Error> {
    sqlx::migrate!("./migrations")
        .run(pool)
        .await
        .map_err(|e| Error::Database(e.into()))
} 