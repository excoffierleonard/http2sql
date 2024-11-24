use crate::db::DbPool;
use actix_web::{error::ResponseError, get, http::StatusCode, web, Responder, Result};
use log::warn;
use serde::Serialize;
use sqlx::Row;

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
struct FetchResponse(Vec<serde_json::Value>);

#[derive(Serialize)]
struct ExecuteResponse {
    rows_affected: u64,
    last_insert_id: u64,
}

#[derive(Serialize)]
struct ErrorResponse {
    message: String,
}

#[get("/v1/test")]
pub async fn test(pool: web::Data<DbPool>) -> Result<impl Responder, ApiError> {
    let rows = pool
        .query_fetch("SELECT JSON_OBJECT('time', NOW()) as json")
        .await
        .map_err(|e| {
            warn!("Database error: {}", e);
            ApiError::Database(e)
        })?;

    let json_rows = rows
        .iter()
        .filter_map(|row| row.try_get::<serde_json::Value, _>("json").ok())
        .collect();

    Ok(web::Json(FetchResponse(json_rows)))
}

#[get("/v1/test_execute")]
pub async fn test_execute(pool: web::Data<DbPool>) -> Result<impl Responder, ApiError> {
    let result = pool
        .query_execute("CREATE TABLE IF NOT EXISTS test (id INT)")
        .await
        .map_err(|e| {
            warn!("Database error: {}", e);
            ApiError::Database(e)
        })?;

    Ok(web::Json(ExecuteResponse {
        rows_affected: result.rows_affected(),
        last_insert_id: result.last_insert_id(),
    }))
}
