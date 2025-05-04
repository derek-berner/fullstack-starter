use std::sync::Arc;
use std::net::IpAddr;
use dotenv::dotenv;
use env_logger::Env;
use warp::Filter;
use backend::{config::Config, db::{create_pool, run_migrations}, error::handle_rejection, routes::api_routes, AppState};
use sqlx::postgres::PgPoolOptions;
use aws_config::BehaviorVersion;
use aws_sdk_s3::Client;
use crate::s3::{read_timestamp, write_timestamp};

mod config;
mod error;
mod routes;
mod db;
mod models;
mod s3;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: sqlx::PgPool,
    pub s3_client: aws_sdk_s3::Client,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables
    dotenv().ok();
    
    // Initialize logger
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    // Load configuration
    let config = Config::from_env()?;
    
    // Initialize database pool
    let pool = create_pool(&config.database_url).await?;
    
    // Run migrations
    run_migrations(&pool).await?;
    
    // Initialize S3 client
    let endpoint_url = std::env::var("AWS_ENDPOINT_URL").unwrap_or_else(|_| "http://localstack:4566".to_string());
    println!("Configuring S3 client with endpoint: {}", endpoint_url);
    
    let s3_config = aws_config::defaults(BehaviorVersion::latest())
        .endpoint_url(&endpoint_url)
        .region(std::env::var("AWS_REGION").unwrap_or_else(|_| "us-east-1".to_string()))
        .credentials_provider(aws_config::Credentials::new(
            std::env::var("AWS_ACCESS_KEY_ID").unwrap_or_else(|_| "test".to_string()),
            std::env::var("AWS_SECRET_ACCESS_KEY").unwrap_or_else(|_| "test".to_string()),
            None,
            None,
            "static"
        ))
        .timeout_config(aws_config::timeout::TimeoutConfig::builder()
            .connect_timeout(std::time::Duration::from_secs(30))
            .read_timeout(std::time::Duration::from_secs(30))
            .api_call_timeout(std::time::Duration::from_secs(30))
            .api_call_attempt_timeout(std::time::Duration::from_secs(30))
            .build())
        .force_path_style(true)
        .no_verify_ssl(true)
        .load()
        .await;
    let s3_client = Client::new(&s3_config);

    println!("Testing S3 connection...");
    // Test S3 connection by listing buckets
    match s3_client.list_buckets().send().await {
        Ok(buckets) => println!("Successfully connected to S3. Buckets: {:?}", buckets.buckets()),
        Err(e) => println!("Error connecting to S3: {}", e),
    }

    // Create S3 bucket if it doesn't exist
    println!("Creating test bucket...");
    match s3_client.create_bucket()
        .bucket("test-bucket")
        .send()
        .await {
        Ok(_) => println!("Created S3 bucket 'test-bucket'"),
        Err(e) => println!("Bucket might already exist: {}", e),
    }

    // Create shared state
    let state = Arc::new(AppState {
        db_pool: pool,
        s3_client,
    });

    // Create routes
    let routes = api_routes(state.clone())
        .or(warp::path("timestamp")
            .and(warp::post())
            .and(warp::any().map(move || state.clone()))
            .and_then(|state: Arc<AppState>| async move {
                match write_timestamp(&state.s3_client).await {
                    Ok(timestamp) => Ok(warp::reply::json(&serde_json::json!({
                        "status": "ok",
                        "timestamp": timestamp
                    }))),
                    Err(e) => Ok(warp::reply::json(&serde_json::json!({
                        "status": "error",
                        "message": e.to_string()
                    }))),
                }
            }))
        .or(warp::path("timestamp")
            .and(warp::get())
            .and(warp::any().map(move || state.clone()))
            .and_then(|state: Arc<AppState>| async move {
                match read_timestamp(&state.s3_client).await {
                    Ok(content) => Ok(warp::reply::json(&serde_json::json!({
                        "status": "ok",
                        "content": content
                    }))),
                    Err(e) => Ok(warp::reply::json(&serde_json::json!({
                        "status": "error",
                        "message": e.to_string()
                    }))),
                }
            }))
        .recover(handle_rejection)
        .with(warp::log("api"));

    println!("Server starting on {}:{}", config.host, config.port);
    
    // Parse host address
    let host: IpAddr = config.host.parse()?;
    
    // Start server
    warp::serve(routes)
        .run((host, config.port))
        .await;

    Ok(())
} 