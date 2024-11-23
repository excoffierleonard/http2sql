use actix_web::{get, web, Responder, Result};
use log::warn;
use serde::Serialize;

use crate::db::{self, DbPool};

#[derive(Serialize)]
pub struct Response {
    message: String,
    db_timestamp: String,
}

#[get("/v1/test")]
pub async fn test(pool: web::Data<DbPool>) -> Result<impl Responder> {
    let pool = pool.get_pool().await.map_err(|e| {
        warn!("Database connection error: {}", e);
        actix_web::error::ErrorInternalServerError(e)
    })?;

    let timestamp = db::get_current_timestamp(&pool).await.map_err(|e| {
        warn!("Database query error: {}", e);
        actix_web::error::ErrorInternalServerError(e)
    })?;

    Ok(web::Json(Response {
        message: "Test Worked".to_string(),
        db_timestamp: timestamp.to_string(),
    }))
}
