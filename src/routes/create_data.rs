use crate::{db::DbPool, errors::ApiError, responses::ApiResponse, utils::auth::ApiKey};
use actix_web::{
    post,
    web::{Data, Json},
    Result,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as, types::chrono::NaiveDateTime};
use uuid::Uuid;

#[derive(Deserialize, Debug)]
struct RequestBody {
    api_key: String,
    name: String,
}

#[derive(Serialize, Debug)]
struct ResponseData {
    uuid: String,
    user_uuid: String,
    name: String,
    created_at: NaiveDateTime,
}

#[post("/tags")]
async fn create_tags(
    pool: Data<DbPool>,
    request_body: Json<RequestBody>,
) -> Result<ApiResponse<ResponseData>, ApiError> {
    // Auth
    let user_uuid = api_key_auth(&pool, &request_body.api_key).await?;

    let uuid = Uuid::new_v4().to_string();

    // First do the insert
    query!(
        "INSERT INTO tags (uuid, user_uuid, name) VALUES (?, ?, ?)",
        &uuid,
        &user_uuid,
        &request_body.name
    )
    .execute(pool.get_pool())
    .await?;

    // Then get the inserted row
    let tags_metadata = query_as!(
        ResponseData,
        "SELECT uuid, user_uuid, name, created_at 
        FROM tags WHERE uuid = ?
        ",
        uuid,
    )
    .fetch_one(pool.get_pool())
    .await?;

    Ok(ApiResponse::new(
        Some(tags_metadata),
        Some("Tag created successfully".to_string()),
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
