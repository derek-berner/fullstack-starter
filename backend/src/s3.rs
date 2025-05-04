use aws_sdk_s3::Client;
use chrono::Utc;
use crate::error::Error;
use reqwest;

const BUCKET_NAME: &str = "test-bucket";
const OBJECT_KEY: &str = "timestamp.txt";

pub async fn write_timestamp(client: &Client) -> Result<String, Error> {
    let timestamp = Utc::now().to_rfc3339();
    let content = format!("Current timestamp: {}", timestamp);
    
    println!("Attempting to write to S3 bucket: {}", BUCKET_NAME);
    
    // Try direct HTTP request for debugging
    let endpoint_url = std::env::var("AWS_ENDPOINT_URL").unwrap_or_else(|_| "http://localstack:4566".to_string());
    let url = format!("{}/{}/{}", endpoint_url, BUCKET_NAME, OBJECT_KEY);
    
    println!("Debug: Trying direct HTTP PUT to {}", url);
    let client = reqwest::Client::new();
    match client.put(&url)
        .header("Content-Type", "text/plain")
        .body(content.clone())
        .send()
        .await {
        Ok(response) => {
            let status = response.status();
            println!("Debug: HTTP response status: {}", status);
            if status.is_success() {
                Ok(timestamp)
            } else {
                let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
                println!("Debug: Error response: {}", error_text);
                Err(Error::S3(format!("HTTP error: {}", status)))
            }
        },
        Err(e) => {
            println!("Debug: HTTP request error: {}", e);
            Err(Error::S3(e.to_string()))
        }
    }
}

pub async fn read_timestamp(client: &Client) -> Result<String, Error> {
    println!("Attempting to read from S3 bucket: {}", BUCKET_NAME);
    
    // Try direct HTTP request for debugging
    let endpoint_url = std::env::var("AWS_ENDPOINT_URL").unwrap_or_else(|_| "http://localstack:4566".to_string());
    let url = format!("{}/{}/{}", endpoint_url, BUCKET_NAME, OBJECT_KEY);
    
    println!("Debug: Trying direct HTTP GET to {}", url);
    let client = reqwest::Client::new();
    match client.get(&url)
        .send()
        .await {
        Ok(response) => {
            let status = response.status();
            println!("Debug: HTTP response status: {}", status);
            if status.is_success() {
                let content = response.text().await.map_err(|e| Error::S3(e.to_string()))?;
                Ok(content)
            } else {
                let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
                println!("Debug: Error response: {}", error_text);
                Err(Error::S3(format!("HTTP error: {}", status)))
            }
        },
        Err(e) => {
            println!("Debug: HTTP request error: {}", e);
            Err(Error::S3(e.to_string()))
        }
    }
} 