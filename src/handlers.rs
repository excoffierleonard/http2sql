use crate::db::{self, LazyPool};
use actix_web::{get, web, Responder, Result};
use log::{debug, warn};
use serde::Serialize;

#[derive(Serialize)]
pub struct Response {
    message: String,
    db_timestamp: String,
}

#[get("/v1/test")]
pub async fn test(lazy_pool: web::Data<LazyPool>) -> Result<impl Responder> {
    let pool = lazy_pool.get_pool().await.map_err(|e| {
        warn!("Database connection error: {}", e);
        actix_web::error::ErrorInternalServerError(e)
    })?;

    let timestamp = db::get_current_timestamp(&pool).await.map_err(|e| {
        warn!("Database query error: {}", e);
        actix_web::error::ErrorInternalServerError(e)
    })?;

    debug!("Successfully fetched timestamp: {}", timestamp);

    let response = Response {
        message: "Test Worked".to_string(),
        db_timestamp: timestamp.to_string(),
    };

    Ok(web::Json(response))
}
