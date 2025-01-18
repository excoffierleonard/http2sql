use crate::{db::DbPool, errors::ApiError, responses::ApiResponse};
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
    user_uuid: String,
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

    // First do the insert
    query!(
        "INSERT INTO tags (uuid, user_uuid, name) VALUES (?, ?, ?)",
        uuid,
        &request_body.user_uuid,
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
