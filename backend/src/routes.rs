use warp::Filter;
use crate::AppState;
use std::sync::Arc;
use sqlx::PgPool;
use crate::models::{Message, PaginatedMessages, PaginationParams};
use crate::error::Error;
use crate::s3::{write_timestamp, read_timestamp};

pub fn health_check() -> impl warp::Reply {
    warp::reply::json(&serde_json::json!({
        "status": "ok"
    }))
}

async fn get_messages(
    pool: &sqlx::PgPool,
    page: i64,
    per_page: i64,
) -> Result<impl warp::Reply, warp::Rejection> {
    let offset = (page - 1) * per_page;
    
    // Get total count
    let total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM messages")
        .fetch_one(pool)
        .await
        .map_err(|e| {
            eprintln!("Error counting messages: {}", e);
            warp::reject::custom(Error::Database(e))
        })?;

    // Get paginated messages
    let messages = sqlx::query_as::<_, Message>(
        "SELECT id, content, author, created_at FROM messages ORDER BY created_at DESC LIMIT $1 OFFSET $2"
    )
    .bind(per_page)
    .bind(offset)
    .fetch_all(pool)
    .await
    .map_err(|e| {
        eprintln!("Error fetching messages: {}", e);
        warp::reject::custom(Error::Database(e))
    })?;

    Ok(warp::reply::json(&PaginatedMessages {
        messages,
        total,
        page,
        per_page,
    }))
}

async fn write_timestamp_handler(state: Arc<AppState>) -> Result<impl warp::Reply, warp::Rejection> {
    match write_timestamp(&state.s3_client).await {
        Ok(timestamp) => Ok(warp::reply::json(&serde_json::json!({
            "status": "success",
            "message": "Timestamp written to S3",
            "timestamp": timestamp
        }))),
        Err(e) => {
            eprintln!("Error writing to S3: {}", e);
            Err(warp::reject::custom(Error::S3(e.to_string())))
        }
    }
}

async fn read_timestamp_handler(state: Arc<AppState>) -> Result<impl warp::Reply, warp::Rejection> {
    match read_timestamp(&state.s3_client).await {
        Ok(content) => Ok(warp::reply::json(&serde_json::json!({
            "timestamp": content
        }))),
        Err(e) => {
            eprintln!("Error reading from S3: {}", e);
            Err(warp::reject::custom(Error::S3(e.to_string())))
        }
    }
}

pub fn api_routes(
    state: Arc<AppState>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let state_clone = Arc::clone(&state);
    let messages_route = warp::path!("messages")
        .and(warp::get())
        .and(warp::query::<PaginationParams>())
        .and_then(move |query: PaginationParams| {
            let pool = state_clone.db_pool.clone();
            async move {
                get_messages(
                    &pool,
                    query.page.unwrap_or(1),
                    query.per_page.unwrap_or(10),
                )
                .await
            }
        });

    let state_clone = Arc::clone(&state);
    let write_timestamp_route = warp::path!("timestamp")
        .and(warp::post())
        .and(warp::any().map(move || Arc::clone(&state_clone)))
        .and_then(write_timestamp_handler);

    let state_clone = Arc::clone(&state);
    let read_timestamp_route = warp::path!("timestamp")
        .and(warp::get())
        .and(warp::any().map(move || Arc::clone(&state_clone)))
        .and_then(read_timestamp_handler);

    let health_route = warp::path!("health")
        .and(warp::get())
        .map(health_check);

    messages_route
        .or(write_timestamp_route)
        .or(read_timestamp_route)
        .or(health_route)
        .with(warp::cors().allow_any_origin())
} 