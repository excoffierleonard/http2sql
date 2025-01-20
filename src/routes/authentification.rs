use crate::{db::DbPool, errors::ApiError, responses::ApiResponse, utils::auth::Password};
use actix_web::{
    post,
    web::{Data, Json},
    Result,
};
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as, types::chrono::NaiveDateTime};
use uuid::Uuid;

#[derive(Deserialize, Debug)]
struct Credentials {
    email: String,
    password: String,
}

#[derive(Serialize, Debug)]
struct Metadata {
    uuid: String,
    email: String,
    created_at: NaiveDateTime,
}

#[post("/auth/sign-up")]
async fn sign_up(
    pool: Data<DbPool>,
    request_body: Json<Credentials>,
) -> Result<ApiResponse<Metadata>, ApiError> {
    let hashed_password = Password::new(&request_body.password).validate()?.hash()?;

    let uuid = Uuid::new_v4().to_string();

    // First do the insert
    query!(
        "INSERT INTO users (uuid, email, password) VALUES (?, ?, ?)",
        uuid,
        &request_body.email,
        hashed_password
    )
    .execute(pool.get_pool())
    .await?;

    // Then get the inserted row
    let user_metadata = query_as!(
        Metadata,
        "SELECT uuid, email, created_at FROM users WHERE uuid = ?",
        uuid
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

#[post("/auth/sign-in")]
async fn sign_in(
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
        // TODO: Add handling for correct login
        true => Ok(ApiResponse::new(None, Some("Correct password".to_string()))),
        false => Err(ApiError::Unauthorized("Invalid password".to_string())),
    }
}
