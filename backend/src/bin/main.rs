use std::sync::Arc;
use std::net::IpAddr;
use dotenv::dotenv;
use env_logger::Env;
use warp::Filter;
use backend::{AppState, Config, handle_rejection, api_routes};
use aws_sdk_s3::Client;
use aws_types::region::Region;
use aws_config::default_provider::credentials::DefaultCredentialsChain;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables
    dotenv().ok();
    
    // Initialize logger
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    // Load configuration
    let config = Config::from_env()?;
    
    // Create database pool
    let pool = backend::db::create_pool(&config.database_url).await?;
    
    // Run migrations
    backend::db::run_migrations(&pool).await?;
    
    // Initialize S3 client
    let endpoint_url = std::env::var("AWS_ENDPOINT_URL").unwrap_or_else(|_| "http://localhost:4566".to_string());
    let region = match std::env::var("AWS_REGION") {
        Ok(r) => r,
        Err(_) => "us-east-1".to_string(),
    };

    let s3_config = aws_config::from_env()
        .endpoint_url(&endpoint_url)
        .region(Region::new(region))
        .load()
        .await;
    let s3_client = Client::new(&s3_config);

    // Create shared state
    let state = Arc::new(AppState {
        db_pool: pool,
        s3_client,
    });

    // Create routes
    let routes = api_routes(state)
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