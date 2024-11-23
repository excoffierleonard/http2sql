use actix_web::{error::ResponseError, get, http::StatusCode, web, Responder, Result};
use log::warn;
use serde::Serialize;

use crate::db::{self, DbPool};

#[derive(Debug)]
pub enum ApiError {
    Database(sqlx::Error),
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Database(e) => write!(f, "Database error: {}", e),
        }
    }
}

impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        let error_response = ErrorResponse {
            message: self.to_string(),
        };
        actix_web::HttpResponse::build(self.status_code()).json(error_response)
    }
}

#[derive(Serialize)]
struct Response {
    message: String,
    db_timestamp: String,
}

#[derive(Serialize)]
struct ErrorResponse {
    message: String,
}

#[get("/v1/test")]
pub async fn test(pool: web::Data<DbPool>) -> Result<impl Responder, ApiError> {
    let pool = pool.get_pool().await.map_err(|e| {
        warn!("Database connection error: {}", e);
        ApiError::Database(e)
    })?;

    let timestamp = db::get_current_timestamp(&pool).await.map_err(|e| {
        warn!("Database query error: {}", e);
        ApiError::Database(e)
    })?;

    Ok(web::Json(Response {
        message: "Test Worked".to_string(),
        db_timestamp: timestamp.to_string(),
    }))
}
