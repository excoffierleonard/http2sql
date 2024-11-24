use crate::db::DbPool;
use actix_web::{error::ResponseError, get, http::StatusCode, post, web, Responder, Result};
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
struct TestItem {
    id: i32,
    name: String,
}

#[derive(Serialize)]
struct FetchResponse(Vec<TestItem>);

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
pub async fn test_fetch(pool: web::Data<DbPool>) -> Result<impl Responder, ApiError> {
    let result = pool
        .query_fetch("SELECT id, name FROM test")
        .await
        .map_err(|e| {
            warn!("Database error: {}", e);
            ApiError::Database(e)
        })?;

    let items: Vec<TestItem> = result
        .iter()
        .filter_map(|row| {
            let id: i32 = row.try_get("id").ok()?;
            let name: String = row.try_get("name").ok()?;
            Some(TestItem { id, name })
        })
        .collect();

    Ok(web::Json(FetchResponse(items)))
}

#[post("/v1/test")]
pub async fn test_execute(pool: web::Data<DbPool>) -> Result<impl Responder, ApiError> {
    let result = pool
        .query_execute("CREATE TABLE IF NOT EXISTS test (id INT, name TEXT, PRIMARY KEY (id))")
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
