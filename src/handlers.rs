use crate::db;
use actix_web::{get, web, Responder};
use serde::Serialize;
use sqlx::MySqlPool;

#[derive(Serialize)]
pub struct Response {
    message: String,
    db_timestamp: String,
}

#[get("/v1/test")]
pub async fn test(pool: web::Data<MySqlPool>) -> impl Responder {
    let timestamp = db::get_current_timestamp(pool.get_ref()).await.unwrap();

    let response = Response {
        message: "Test Worked".to_string(),
        db_timestamp: timestamp.to_string(),
    };

    web::Json(response)
}
