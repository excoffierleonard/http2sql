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
use serde_json::Value;

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

#[derive(Deserialize)]
struct TableRow(std::collections::HashMap<String, Value>);

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

#[post("/v1/tables/{table_name}/rows")]
pub async fn insert_rows(
    pool: web::Data<DbPool>,
    table_name: Path<String>,
    payload: web::Json<Vec<TableRow>>,
) -> Result<impl Responder, ApiError> {
    let pool = pool.get_pool().await.map_err(|e| {
        warn!("Database error: {}", e);
        ApiError::Database(e)
    })?;

    let table_name = table_name.into_inner();
    if table_name.is_empty() {
        return Err(ApiError::InvalidInput("Table name is required".to_string()));
    }

    if payload.is_empty() {
        return Err(ApiError::InvalidInput(
            "At least one row is required".to_string(),
        ));
    }

    // Get column names from the first row
    let columns: Vec<String> = payload[0].0.keys().cloned().collect();
    if columns.is_empty() {
        return Err(ApiError::InvalidInput(
            "Row data cannot be empty".to_string(),
        ));
    }

    // Build the placeholders for MySQL (use ? instead of $1, $2, etc.)
    let placeholders = vec!["?"; columns.len()];

    // Construct the base query
    let query = format!(
        "INSERT INTO {} ({}) VALUES ({})",
        table_name,
        columns.join(", "),
        placeholders.join(", ")
    );

    // Execute insert for each row
    for row in payload.iter() {
        let mut query_builder = sqlx::query(&query);

        // Bind each value in the correct order
        for column in &columns {
            query_builder = query_builder.bind(row.0.get(column).cloned().unwrap_or(Value::Null));
        }

        query_builder.execute(&pool).await.map_err(|e| {
            warn!("Database error: {}", e);
            ApiError::Database(e)
        })?;
    }

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
