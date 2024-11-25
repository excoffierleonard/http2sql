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
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use log::warn;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{Column, Row, TypeInfo};

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
struct TableColumn {
    name: String,
    data_type: String,
    constraints: Option<Vec<String>>,
}

#[derive(Deserialize)]
struct CreateTableRequest {
    table_name: String,
    columns: Vec<TableColumn>,
}

#[derive(Deserialize, Serialize)]
struct TableRow(std::collections::HashMap<String, Value>);

#[derive(Deserialize)]
pub struct CustomQueryRequest {
    query: String,
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

    // Create placeholders for a single row
    let row_placeholders = vec!["?"; columns.len()];
    let single_row_placeholders = format!("({})", row_placeholders.join(", "));

    // Create placeholders for all rows
    let all_rows_placeholders = vec![single_row_placeholders; payload.len()];

    // Construct the bulk insert query
    let query = format!(
        "INSERT INTO {} ({}) VALUES {}",
        table_name,
        columns.join(", "),
        all_rows_placeholders.join(", ")
    );

    // Create a single query builder
    let mut query_builder = sqlx::query(&query);

    // Bind all values from all rows in order
    for row in payload.iter() {
        for column in &columns {
            query_builder = query_builder.bind(row.0.get(column).cloned().unwrap_or(Value::Null));
        }
    }

    // Execute single bulk insert
    query_builder.execute(&pool).await.map_err(|e| {
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

    let query = format!("DROP TABLE {}", table_name);

    sqlx::query(&query).execute(&pool).await.map_err(|e| {
        warn!("Database error: {}", e);
        ApiError::Database(e)
    })?;

    Ok(HttpResponse::NoContent().finish())
}

#[get("/v1/custom")]
pub async fn custom_query_fetch(
    pool: web::Data<DbPool>,
    query: web::Json<CustomQueryRequest>,
) -> Result<impl Responder, ApiError> {
    let pool = pool.get_pool().await.map_err(|e| {
        warn!("Database error: {}", e);
        ApiError::Database(e)
    })?;

    // Validate that the query is a SELECT statement
    let normalized_query = query.query.trim().to_uppercase();
    if !normalized_query.starts_with("SELECT") {
        return Err(ApiError::InvalidInput(
            "Only SELECT queries are allowed".to_string(),
        ));
    }

    let rows = sqlx::query(&query.query)
        .fetch_all(&pool)
        .await
        .map_err(|e| {
            warn!("Database error: {}", e);
            ApiError::Database(e)
        })?;

    let mut result: Vec<TableRow> = Vec::with_capacity(rows.len());

    for row in rows {
        let mut map = std::collections::HashMap::new();
        let columns = row.columns();

        for column in columns {
            let column_name = column.name();
            let type_info = column.type_info();
            let type_name = type_info.name();

            // Convert the value based on its SQL type
            let value = match type_name {
                // Integer types
                "TINYINT" | "SMALLINT" | "MEDIUMINT" | "INT" | "BIGINT" => {
                    if type_info.name().contains("UNSIGNED") {
                        row.try_get::<u64, _>(column_name)
                            .ok()
                            .map(Value::from)
                            .unwrap_or(Value::Null)
                    } else {
                        row.try_get::<i64, _>(column_name)
                            .ok()
                            .map(Value::from)
                            .unwrap_or(Value::Null)
                    }
                }
                // Floating point types
                "FLOAT" | "DOUBLE" => row
                    .try_get::<f64, _>(column_name)
                    .ok()
                    .map(Value::from)
                    .unwrap_or(Value::Null),
                // Decimal types - handle as string to preserve precision
                "DECIMAL" => row
                    .try_get::<String, _>(column_name)
                    .ok()
                    .map(Value::from)
                    .unwrap_or(Value::Null),
                // String types
                "CHAR" | "VARCHAR" | "TEXT" | "TINYTEXT" | "MEDIUMTEXT" | "LONGTEXT" => row
                    .try_get::<String, _>(column_name)
                    .ok()
                    .map(Value::from)
                    .unwrap_or(Value::Null),
                // Binary types
                "BINARY" | "VARBINARY" | "BLOB" | "TINYBLOB" | "MEDIUMBLOB" | "LONGBLOB" => row
                    .try_get::<Vec<u8>, _>(column_name)
                    .ok()
                    .map(|bytes| Value::String(base64::encode(&bytes)))
                    .unwrap_or(Value::Null),
                // Boolean type (typically stored as TINYINT(1))
                "BOOL" | "BOOLEAN" => row
                    .try_get::<bool, _>(column_name)
                    .ok()
                    .map(Value::from)
                    .unwrap_or(Value::Null),
                // Date and Time types
                "DATE" => row
                    .try_get::<NaiveDate, _>(column_name)
                    .ok()
                    .map(|d| Value::String(d.format("%Y-%m-%d").to_string()))
                    .unwrap_or(Value::Null),
                "TIME" => row
                    .try_get::<NaiveTime, _>(column_name)
                    .ok()
                    .map(|t| Value::String(t.format("%H:%M:%S").to_string()))
                    .unwrap_or(Value::Null),
                "DATETIME" | "TIMESTAMP" => row
                    .try_get::<NaiveDateTime, _>(column_name)
                    .ok()
                    .map(|dt| Value::String(dt.format("%Y-%m-%d %H:%M:%S").to_string()))
                    .unwrap_or(Value::Null),
                // JSON type
                "JSON" => row
                    .try_get::<Value, _>(column_name)
                    .ok()
                    .unwrap_or(Value::Null),
                // Enum and Set types (treated as strings)
                "ENUM" | "SET" => row
                    .try_get::<String, _>(column_name)
                    .ok()
                    .map(Value::from)
                    .unwrap_or(Value::Null),
                // Default case for unknown types
                _ => {
                    warn!(
                        "Unsupported type: {} for column: {}",
                        type_name, column_name
                    );
                    Value::Null
                }
            };

            map.insert(column_name.to_string(), value);
        }

        result.push(TableRow(map));
    }

    Ok(HttpResponse::Ok().json(result))
}

#[post("/v1/custom")]
pub async fn custom_query_execute(
    pool: web::Data<DbPool>,
    query: web::Json<CustomQueryRequest>,
) -> Result<impl Responder, ApiError> {
    let pool = pool.get_pool().await.map_err(|e| {
        warn!("Database error: {}", e);
        ApiError::Database(e)
    })?;

    // Validate that the query is NOT a SELECT statement since we have GET for that
    let normalized_query = query.query.trim().to_uppercase();
    if normalized_query.starts_with("SELECT") {
        return Err(ApiError::InvalidInput(
            "SELECT queries should use GET method instead".to_string(),
        ));
    }

    // Execute the query without fetching results
    sqlx::query(&query.query)
        .execute(&pool)
        .await
        .map_err(|e| {
            warn!("Database error: {}", e);
            ApiError::Database(e)
        })?;

    // Return 201 Created for operations that create new resources
    if normalized_query.starts_with("INSERT") || normalized_query.starts_with("CREATE") {
        Ok(HttpResponse::Created().finish())
    } else {
        Ok(HttpResponse::Ok().finish())
    }
}
