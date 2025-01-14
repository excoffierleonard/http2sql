use crate::{auth::Password, db::DbPool, errors::ApiError, responses::ApiResponse};
use actix_web::{
    post,
    web::{Data, Json},
    Result,
};
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as, types::chrono::NaiveDateTime};

#[derive(Deserialize, Debug)]
struct Credentials {
    email: String,
    password: String,
}

#[derive(Serialize, Debug)]
struct Metadata {
    id: i32,
    email: String,
    created_at: NaiveDateTime,
}

#[post("/auth/register")]
async fn register_user(
    pool: Data<DbPool>,
    request_body: Json<Credentials>,
) -> Result<ApiResponse<Metadata>, ApiError> {
    let hashed_password = Password::new(&request_body.password).validate()?.hash()?;

    // First do the insert
    query!(
        "INSERT INTO users (email, password) VALUES (?, ?)",
        &request_body.email,
        hashed_password
    )
    .execute(pool.get_pool())
    .await?;

    // Then get the inserted row
    let user_metadata = query_as!(
        Metadata,
        "SELECT id, email, created_at FROM users WHERE email = ?",
        &request_body.email
    )
    .fetch_one(pool.get_pool())
    .await?;

    Ok(ApiResponse::new(
        Some(user_metadata),
        Some("User registered successfully".to_string()),
    ))
}

#[derive(Serialize, Debug)]
struct DbPassword {
    password: String,
}

#[post("/auth/login")]
async fn login_user(
    pool: Data<DbPool>,
    request_body: Json<Credentials>,
) -> Result<ApiResponse<()>, ApiError> {
    let db_password = query_as!(
        DbPassword,
        "
        SELECT password 
        FROM users WHERE email = ?
        ",
        &request_body.email
    )
    .fetch_one(pool.get_pool())
    .await?;

    match Password::new(&request_body.password)
        .validate()?
        .verify(&db_password.password)?
    {
        true => Ok(ApiResponse::new(None, Some("Correct password".to_string()))),
        false => Err(ApiError::Unauthorized("Invalid password".to_string())),
    }
}
