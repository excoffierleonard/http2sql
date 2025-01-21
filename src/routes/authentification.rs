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
struct UserMetadata {
    uuid: String,
    email: String,
    created_at: NaiveDateTime,
}

#[post("/auth/sign-up")]
async fn sign_up(
    pool: Data<DbPool>,
    request_body: Json<Credentials>,
) -> Result<ApiResponse<UserMetadata>, ApiError> {
    // Validate the password
    let password = Password::new(&request_body.password)?;

    // Register the user in the database
    let user_metadata = register_user_in_db(&pool, &request_body.email, &password).await?;

    Ok(ApiResponse::new(
        Some(user_metadata),
        Some("User registered successfully".to_string()),
    ))
}

async fn register_user_in_db(
    pool: &DbPool,
    email: &str,
    password: &Password,
) -> Result<UserMetadata, ApiError> {
    let uuid = Uuid::new_v4().to_string();

    let hashed_password = password.hash()?;

    // First do the insert
    query!(
        "INSERT INTO users (uuid, email, password_hash) VALUES (?, ?, ?)",
        uuid,
        email,
        hashed_password
    )
    .execute(pool.get_pool())
    .await?;

    // Then get the inserted row
    let user_metadata = query_as!(
        UserMetadata,
        "SELECT uuid, email, created_at FROM users WHERE uuid = ?",
        uuid
    )
    .fetch_one(pool.get_pool())
    .await?;

    Ok(user_metadata)
}

#[derive(Serialize, Debug)]
struct DbSignInResponse {
    uuid: String,
    password_hash: String,
}

#[derive(Debug)]
struct VerifiedUser {
    uuid: String,
}

#[derive(Serialize, Debug)]
struct ApiKeyResponse {
    user_uuid: String,
    api_key: String,
    created_at: NaiveDateTime,
    expires_at: Option<NaiveDateTime>,
}

#[post("/auth/sign-in")]
async fn sign_in(
    pool: Data<DbPool>,
    request_body: Json<Credentials>,
) -> Result<ApiResponse<ApiKeyResponse>, ApiError> {
    // Verify user credentials
    let password = Password::new(&request_body.password)?;
    let verified_user = verify_user_credentials(&pool, &request_body.email, &password).await?;

    // Generate and store API key
    let api_key = ApiKey::generate();
    let api_key_metadata = store_api_key(&pool, &verified_user.uuid, &api_key).await?;

    // Return success response
    Ok(ApiResponse::new(
        Some(ApiKeyResponse {
            user_uuid: verified_user.uuid,
            api_key: api_key.into_string(),
            created_at: api_key_metadata.created_at,
            expires_at: api_key_metadata.expires_at,
        }),
        Some("Password is correct, API key generated successfully".to_string()),
    ))
}

// This function handles the database query and password verification
async fn verify_user_credentials(
    pool: &DbPool,
    email: &str,
    password: &Password,
) -> Result<VerifiedUser, ApiError> {
    // Query the database for user credentials
    let db_sign_in_response = query_as!(
        DbSignInResponse,
        "
        SELECT uuid, password_hash 
        FROM users WHERE email = ?
        ",
        email
    )
    .fetch_one(pool.get_pool())
    .await?;

    // Verify the password - if verification fails, this will return early with an error
    password.verify(&db_sign_in_response.password_hash)?;

    // If we get here, password verification succeeded
    Ok(VerifiedUser {
        uuid: db_sign_in_response.uuid,
    })
}

#[derive(Serialize, Debug)]
struct ApiKeyMetadata {
    created_at: NaiveDateTime,
    expires_at: Option<NaiveDateTime>,
}

// Store the API key in the database
async fn store_api_key(
    pool: &DbPool,
    user_uuid: &str,
    api_key: &ApiKey,
) -> Result<ApiKeyMetadata, ApiError> {
    let uuid = Uuid::new_v4().to_string();

    let api_key_hash = api_key.hash();

    // Store the API key in the database
    query!(
        "INSERT INTO api_keys (uuid, user_uuid, api_key_hash) VALUES (?, ?, ?)",
        uuid,
        user_uuid,
        api_key_hash,
    )
    .execute(pool.get_pool())
    .await?;

    // Get the metadata of the stored API key
    let api_key_metadata = query_as!(
        ApiKeyMetadata,
        "SELECT created_at, expires_at FROM api_keys WHERE uuid = ?",
        uuid
    )
    .fetch_one(pool.get_pool())
    .await?;

    Ok(api_key_metadata)
}
