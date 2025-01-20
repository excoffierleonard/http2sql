use crate::{
    db::DbPool,
    errors::ApiError,
    responses::ApiResponse,
    utils::auth::{ApiKey, Password},
};
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
        "INSERT INTO users (uuid, email, password_hash) VALUES (?, ?, ?)",
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
struct DbSignInResponse {
    uuid: String,
    password_hash: String,
}

#[derive(Serialize, Debug)]
struct ApiKeyResponse {
    api_key: String,
}

#[post("/auth/sign-in")]
async fn sign_in(
    pool: Data<DbPool>,
    request_body: Json<Credentials>,
) -> Result<ApiResponse<ApiKeyResponse>, ApiError> {
    let db_sign_in_response = query_as!(
        DbSignInResponse,
        "
        SELECT uuid, password_hash 
        FROM users WHERE email = ?
        ",
        &request_body.email
    )
    .fetch_one(pool.get_pool())
    .await?;

    match Password::new(&request_body.password)
        .validate()?
        .verify(&db_sign_in_response.password_hash)?
    {
        true => {
            let api_key = ApiKey::generate();

            store_api_key(&pool, &db_sign_in_response.uuid, &api_key).await?;

            Ok(ApiResponse::new(
                Some(ApiKeyResponse {
                    api_key: api_key.into_string(),
                }),
                Some("Password is correct, API key generated successfully".to_string()),
            ))
        }
        false => Err(ApiError::Unauthorized("Invalid password".to_string())),
    }
}

async fn store_api_key(pool: &DbPool, user_uuid: &str, api_key: &ApiKey) -> Result<(), ApiError> {
    let uuid = Uuid::new_v4().to_string();

    let api_key_hash = api_key.hash();

    query!(
        "INSERT INTO api_keys (uuid, user_uuid, api_key_hash) VALUES (?, ?, ?)",
        uuid,
        user_uuid,
        api_key_hash,
    )
    .execute(pool.get_pool())
    .await?;

    Ok(())
}
