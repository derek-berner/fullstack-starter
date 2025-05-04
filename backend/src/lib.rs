pub mod config;
pub mod error;
pub mod routes;
pub mod db;
pub mod models;
pub mod s3;

use std::sync::Arc;
use sqlx::PgPool;
use aws_sdk_s3::Client;

pub struct AppState {
    pub db_pool: PgPool,
    pub s3_client: Client,
}

pub type SharedState = Arc<AppState>;

// Re-export commonly used types
pub use crate::config::Config;
pub use crate::error::{Error, handle_rejection};
pub use crate::routes::api_routes; 