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
struct ErrorResponse {
    message: String,
}

#[derive(Serialize)]
struct FetchResponse(Vec<TestItem>);

#[derive(Serialize)]
struct ExecuteResponse {
    rows_affected: u64,
    last_insert_id: u64,
}

#[derive(Serialize)]
struct TestItem {
    id: i32,
    name: String,
    email: String,
}

#[get("/v1/test")]
pub async fn test_fetch(pool: web::Data<DbPool>) -> Result<impl Responder, ApiError> {
    let pool = pool.get_pool().await.map_err(|e| {
        warn!("Database error: {}", e);
        ApiError::Database(e)
    })?;

    let result = sqlx::query(
        "
        SELECT id, name, email
        FROM test;
    ",
    )
    .fetch_all(&pool)
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
            let email: String = row.try_get("email").ok()?;
            Some(TestItem { id, name, email })
        })
        .collect();

    Ok(web::Json(FetchResponse(items)))
}

#[post("/v1/test")]
pub async fn test_execute(pool: web::Data<DbPool>) -> Result<impl Responder, ApiError> {
    let pool = pool.get_pool().await.map_err(|e| {
        warn!("Database error: {}", e);
        ApiError::Database(e)
    })?;

    let result = sqlx::query(
        "
        CREATE TABLE IF NOT EXISTS test (
            id INT AUTO_INCREMENT, 
            name VARCHAR(255), 
            email VARCHAR(255), 
            PRIMARY KEY (id)
        );
    ",
    )
    .execute(&pool)
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
