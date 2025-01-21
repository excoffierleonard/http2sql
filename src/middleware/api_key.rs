use crate::{db::DbPool, errors::ApiError, utils::auth::ApiKey};
use actix_web::Result;
use chrono::Utc;
use sqlx::{query, query_as, types::chrono::NaiveDateTime};

struct ApiKeyMetadata {
    user_uuid: String,
    expires_at: Option<NaiveDateTime>,
}

pub async fn auth_to_uuid(pool: &DbPool, api_key: &str) -> Result<String, ApiError> {
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
