use crate::{db::DbPool, errors::ApiError};
use actix_web::{body::BoxBody, get, web::Data, HttpRequest, HttpResponse, Responder, Result};
use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::query_as;

#[derive(Serialize)]
struct User {
    id: i32,
    email: String,
    password: String,
    created_at: NaiveDateTime,
}

#[derive(Serialize)]
struct Users(Vec<User>);

#[get("/v1/users")]
async fn custom_query(pool: Data<DbPool>) -> Result<Users, ApiError> {
    let pool = pool.get_pool().await?;

    let users = query_as!(
        User,
        "SELECT id, email, password, created_at 
        FROM users;"
    )
    .fetch_all(&pool)
    .await?;

    Ok(Users(users))
}

impl Responder for Users {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        HttpResponse::Ok().json(self)
    }
}
