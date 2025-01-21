use crate::{db::DbPool, errors::ApiError, responses::ApiResponse, utils::auth::ApiKey};
use actix_web::{get, web::Data, Result};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use chrono::Utc;
use serde::Serialize;
use sqlx::{query, query_as, types::chrono::NaiveDateTime};

#[derive(Serialize, Debug)]
struct UserMetadata {
    uuid: String,
    email: String,
    created_at: NaiveDateTime,
}

#[get("/user/metadata")]
async fn get_user_metadata(
    auth: BearerAuth,
    pool: Data<DbPool>,
) -> Result<ApiResponse<UserMetadata>, ApiError> {
    let api_key = auth.token();
    let uuid = api_key_auth(&pool, api_key).await?;

    let user_metadata = query_as!(
        UserMetadata,
        "SELECT uuid, email, created_at FROM users WHERE uuid = ?",
        &uuid,
    )
    .fetch_one(pool.get_pool())
    .await?;

    Ok(ApiResponse::new(
        Some(user_metadata),
        Some("User metadata retrieved successfully".to_string()),
    ))
}

struct ApiKeyMetadata {
    user_uuid: String,
    expires_at: Option<NaiveDateTime>,
}

async fn api_key_auth(pool: &DbPool, api_key: &str) -> Result<String, ApiError> {
    let api_key_hash = ApiKey::new(api_key)?.hash();

    // Get the user_uuid associated with the api_key
    let api_key_metadata = query_as!(
        ApiKeyMetadata,
        "SELECT user_uuid, expires_at FROM api_keys WHERE api_key_hash = ?",
        api_key_hash
    )
    .fetch_one(pool.get_pool())
    .await?;

    // Check if the API key has expired
    if let Some(expires_at) = api_key_metadata.expires_at {
        if expires_at < Utc::now().naive_utc() {
            return Err(ApiError::Unauthorized("API key has expired".to_string()));
        }
    }

    // Update the last_used_at timestamp
    query!(
        "UPDATE api_keys SET last_used_at = CURRENT_TIMESTAMP WHERE api_key_hash = ?",
        api_key_hash
    )
    .execute(pool.get_pool())
    .await?;

    Ok(api_key_metadata.user_uuid)
}
