use crate::db::DbPool;
use actix_web::{
    delete,
    error::ResponseError,
    http::StatusCode,
    post,
    web::{self, Path},
    HttpResponse, Responder, Result,
};
use log::warn;
use serde::{Deserialize, Serialize};

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

#[derive(Deserialize)]
struct Column {
    name: String,
    data_type: String,
    constraints: Option<Vec<String>>,
}

#[derive(Deserialize)]
struct CreateTableRequest {
    table_name: String,
    columns: Vec<Column>,
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

    if payload.table_name.is_empty() {
        return Err(ApiError::InvalidInput("Table name is required".to_string()));
    }

    if payload.columns.is_empty() {
        return Err(ApiError::InvalidInput(
            "At least one column is required".to_string(),
        ));
    }

    let columns: Vec<String> = payload
        .columns
        .iter()
        .map(|col| {
            let mut column_def = format!("{} {}", col.name, col.data_type);

            if let Some(constraints) = &col.constraints {
                if !constraints.is_empty() {
                    column_def = format!("{} {}", column_def, constraints.join(" "));
                }
            }

            column_def
        })
        .collect();

    let query = format!(
        "CREATE TABLE {} ({})",
        payload.table_name,
        columns.join(", ")
    );

    sqlx::query(&query).execute(&pool).await.map_err(|e| {
        warn!("Database error: {}", e);
        ApiError::Database(e)
    })?;

    Ok(HttpResponse::Created().finish())
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

    sqlx::query(&query).execute(&pool).await.map_err(|e| {
        warn!("Database error: {}", e);
        ApiError::Database(e)
    })?;

    Ok(HttpResponse::NoContent().finish())
}
