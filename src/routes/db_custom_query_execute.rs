use crate::{
    db::DbPool,
    errors::ApiError,
    handlers::{execute_query, get_pool, CustomQueryRequest},
};
use actix_web::{
    post,
    web::{Data, Json},
    HttpResponse, Responder, Result,
};

#[post("/v1/custom")]
async fn custom_query_execute(
    pool: Data<DbPool>,
    query: Json<CustomQueryRequest>,
) -> Result<impl Responder, ApiError> {
    handle_custom_query_execute(pool, query).await
}

async fn handle_custom_query_execute(
    pool: Data<DbPool>,
    query: Json<CustomQueryRequest>,
) -> Result<impl Responder, ApiError> {
    let pool = get_pool(pool).await?;

    let normalized_query = query.query.trim().to_uppercase();
    if normalized_query.starts_with("SELECT") {
        return Err(ApiError::InvalidInput(
            "SELECT queries should use GET method instead".to_string(),
        ));
    }

    execute_query(&pool, &query.query).await?;

    if normalized_query.starts_with("INSERT") || normalized_query.starts_with("CREATE") {
        Ok(HttpResponse::Created().finish())
    } else {
        Ok(HttpResponse::Ok().finish())
    }
}
