use crate::{
    db::DbPool,
    errors::ApiError,
    handlers::{execute_query, get_pool, validate_table_name},
};
use actix_web::{
    delete,
    web::{Data, Path},
    HttpResponse, Responder, Result,
};

#[delete("/v1/tables/{table_name}")]
async fn delete_table(
    pool: Data<DbPool>,
    table_name: Path<String>,
) -> Result<impl Responder, ApiError> {
    handle_delete_table(pool, table_name).await
}

async fn handle_delete_table(
    pool: Data<DbPool>,
    table_name: Path<String>,
) -> Result<impl Responder, ApiError> {
    let pool = get_pool(pool).await?;
    let table_name = table_name.into_inner();

    validate_table_name(&table_name)?;

    let query = format!("DROP TABLE {}", table_name);
    execute_query(&pool, &query).await?;

    Ok(HttpResponse::NoContent().finish())
}
