use crate::{
    db::DbPool,
    errors::ApiError,
    handlers::{execute_query, get_pool, validate_table_name},
};
use actix_web::{
    post,
    web::{Data, Json},
    HttpResponse, Responder, Result,
};
use serde::Deserialize;

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

#[post("/v1/tables")]
async fn create_table(
    pool: Data<DbPool>,
    payload: Json<CreateTableRequest>,
) -> Result<impl Responder, ApiError> {
    handle_create_table(pool, payload).await
}

async fn handle_create_table(
    pool: Data<DbPool>,
    payload: Json<CreateTableRequest>,
) -> Result<impl Responder, ApiError> {
    let pool = get_pool(pool).await?;

    if payload.columns.is_empty() {
        return Err(ApiError::InvalidInput(
            "At least one column is required".to_string(),
        ));
    }

    validate_table_name(&payload.table_name)?;

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

    execute_query(&pool, &query).await?;
    Ok(HttpResponse::Created().finish())
}
