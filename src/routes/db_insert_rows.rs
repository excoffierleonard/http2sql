use crate::{
    db::DbPool,
    errors::ApiError,
    handlers::{get_pool, validate_table_name, TableRow},
};
use actix_web::{
    post,
    web::{Data, Json, Path},
    HttpResponse, Responder, Result,
};
use serde_json::Value;
use sqlx::query;

#[post("/v1/tables/{table_name}/rows")]
async fn insert_rows(
    pool: Data<DbPool>,
    table_name: Path<String>,
    payload: Json<Vec<TableRow>>,
) -> Result<impl Responder, ApiError> {
    handle_insert_rows(pool, table_name, payload).await
}

async fn handle_insert_rows(
    pool: Data<DbPool>,
    table_name: Path<String>,
    payload: Json<Vec<TableRow>>,
) -> Result<impl Responder, ApiError> {
    let pool = get_pool(pool).await?;
    let table_name = table_name.into_inner();

    validate_table_name(&table_name)?;

    if payload.is_empty() {
        return Err(ApiError::InvalidInput(
            "At least one row is required".to_string(),
        ));
    }

    let columns: Vec<String> = payload[0].0.keys().cloned().collect();
    if columns.is_empty() {
        return Err(ApiError::InvalidInput(
            "Row data cannot be empty".to_string(),
        ));
    }

    let row_placeholders = vec!["?"; columns.len()];
    let single_row_placeholders = format!("({})", row_placeholders.join(", "));
    let all_rows_placeholders = vec![single_row_placeholders; payload.len()];

    let formated_query = format!(
        "INSERT INTO {} ({}) VALUES {}",
        table_name,
        columns.join(", "),
        all_rows_placeholders.join(", ")
    );

    let mut query_builder = query(&formated_query);
    for row in payload.iter() {
        for column in &columns {
            query_builder = query_builder.bind(row.0.get(column).cloned().unwrap_or(Value::Null));
        }
    }

    query_builder
        .execute(&pool)
        .await
        .map_err(ApiError::Database)?;

    Ok(HttpResponse::Created().finish())
}
