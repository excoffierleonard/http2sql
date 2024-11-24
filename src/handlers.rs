use crate::db::DbPool;
use actix_web::{
    delete,
    error::ResponseError,
    get,
    http::StatusCode,
    post,
    web::{self, Path},
    HttpResponse, Responder, Result,
};
use log::warn;
use serde::{Deserialize, Serialize};
use sqlx::Row;

#[derive(Debug)]
pub enum ApiError {
    Database(sqlx::Error),
    InvalidInput(String),
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Database(e) => write!(f, "Database error: {}", e),
            Self::InvalidInput(e) => write!(f, "Invalid input: {}", e),
        }
    }
}

impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::InvalidInput(_) => StatusCode::BAD_REQUEST,
        }
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

#[derive(Deserialize)]
struct Column {
    name: String,
    data_type: String,
}

#[derive(Deserialize)]
struct CreateTableRequest {
    table_name: String,
    columns: Vec<Column>,
}

#[get("/v1/db")]
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

    Ok(HttpResponse::Ok().json(FetchResponse(items)))
}

#[post("/v1/tables")]
pub async fn create_table(
    pool: web::Data<DbPool>,
    payload: web::Json<CreateTableRequest>,
) -> Result<impl Responder, ApiError> {
    let pool = pool.get_pool().await.map_err(|e| {
        warn!("Database error: {}", e);
        ApiError::Database(e)
    })?;

    if payload.table_name.is_empty() || payload.columns.is_empty() {
        return Err(ApiError::InvalidInput(
            "Table name and at least one column are required".to_string(),
        ));
    }

    let columns: Vec<String> = payload
        .columns
        .iter()
        .enumerate()
        .map(|(i, col)| {
            if i == 0 {
                format!("{} {} AUTO_INCREMENT PRIMARY KEY", col.name, col.data_type)
            } else {
                format!("{} {}", col.name, col.data_type)
            }
        })
        .collect();

    let query = format!(
        "CREATE TABLE IF NOT EXISTS {} ({})",
        payload.table_name,
        columns.join(", ")
    );

    let result = sqlx::query(&query).execute(&pool).await.map_err(|e| {
        warn!("Database error: {}", e);
        ApiError::Database(e)
    })?;

    Ok(HttpResponse::Created().json(ExecuteResponse {
        rows_affected: result.rows_affected(),
        last_insert_id: result.last_insert_id(),
    }))
}

#[delete("/v1/tables/{table_name}")]
pub async fn delete_table(
    pool: web::Data<DbPool>,
    table_name: Path<String>,
) -> Result<impl Responder, ApiError> {
    let pool = pool.get_pool().await.map_err(|e| {
        warn!("Database error: {}", e);
        ApiError::Database(e)
    })?;

    let table_name = table_name.into_inner();
    if table_name.is_empty() {
        return Err(ApiError::InvalidInput("Table name is required".to_string()));
    }

    let query = format!("DROP TABLE IF EXISTS {}", table_name);

    let result = sqlx::query(&query).execute(&pool).await.map_err(|e| {
        warn!("Database error: {}", e);
        ApiError::Database(e)
    })?;

    Ok(HttpResponse::Ok().json(ExecuteResponse {
        rows_affected: result.rows_affected(),
        last_insert_id: result.last_insert_id(),
    }))
}
