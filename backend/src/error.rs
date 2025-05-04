use std::convert::Infallible;
use thiserror::Error;
use warp::{reject::Reject, http::StatusCode, Reply};

#[derive(Error, Debug)]
pub enum Error {
    #[error("database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("S3 error: {0}")]
    S3(String),
}

impl Reject for Error {}

pub async fn handle_rejection(err: warp::Rejection) -> Result<impl Reply, Infallible> {
    let code;
    let message;

    if err.is_not_found() {
        code = StatusCode::NOT_FOUND;
        message = "Not Found";
    } else if let Some(e) = err.find::<Error>() {
        match e {
            Error::Database(_) => {
                code = StatusCode::INTERNAL_SERVER_ERROR;
                message = "Database Error";
            }
            Error::S3(_) => {
                code = StatusCode::INTERNAL_SERVER_ERROR;
                message = "S3 Error";
            }
        }
    } else {
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "Internal Server Error";
    }

    let json = warp::reply::json(&serde_json::json!({
        "code": code.as_u16(),
        "message": message,
    }));

    Ok(warp::reply::with_status(json, code))
} 