use crate::{db::DbPool, errors::ApiError, responses::ApiResponse, utils::auth::ApiKey};
use actix_web::{
    post,
    web::{Data, Json},
    Result,
};
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
    let uuid = Uuid::new_v4().to_string();

    let api_key_hash = ApiKey::new(&request_body.api_key)?.hash();

    // First do the insert
    query!(
        "INSERT INTO tags (uuid, user_uuid, name) VALUES (?, (SELECT user_uuid FROM api_keys WHERE api_key_hash = ?), ?)",
        uuid,
        &api_key_hash,
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
