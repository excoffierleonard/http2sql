use crate::{
    auth::Password, db::DbPool, errors::ApiError, requests::ApiRequest, responses::ApiResponse,
};
use actix_web::{
    post,
    web::{Data, Json},
    Result,
};
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as};

#[derive(Deserialize, Debug)]
struct RequestBody {
    email: String,
    password: String,
}

#[derive(Serialize, Debug)]
struct DbPassword {
    password: String,
}

#[post("/auth/login")]
async fn login_user(
    pool: Data<DbPool>,
    request_body: Json<ApiRequest<RequestBody>>,
) -> Result<ApiResponse<String>, ApiError> {
    let db_password = query_as!(
        DbPassword,
        "SELECT password FROM users WHERE email = ?",
        &request_body.data.email
    )
    .fetch_one(pool.get_pool())
    .await?;

    match Password::new(&request_body.data.password)
        .validate()?
        .verify(&db_password.password)?
    {
        true => Ok(ApiResponse::message("Correct password".to_string())),
        false => Err(ApiError::Unauthorized("Invalid password".to_string())),
    }
}

#[post("/auth/register")]
async fn register_user(
    pool: Data<DbPool>,
    request_body: Json<ApiRequest<RequestBody>>,
) -> Result<ApiResponse<()>, ApiError> {
    let hashed_password = Password::new(&request_body.data.password)
        .validate()?
        .hash()?;

    let query_result = query!(
        "INSERT INTO users (email, password) VALUES (?, ?)",
        &request_body.data.email,
        hashed_password
    )
    .execute(pool.get_pool())
    .await?;

    Ok(ApiResponse::executed(
        query_result.rows_affected(),
        "User registered successfully",
    ))
}
