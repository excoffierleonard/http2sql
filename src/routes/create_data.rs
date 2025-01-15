use crate::{db::DbPool, errors::ApiError, responses::ApiResponse};
use actix_web::{
    post,
    web::{Data, Json},
    Result,
};
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as, types::chrono::NaiveDateTime};

#[derive(Deserialize, Debug)]
struct RequestBody {
    user_id: i32,
    name: String,
}

#[derive(Serialize, Debug)]
struct ResponseData {
    id: i32,
    user_id: i32,
    name: String,
    created_at: NaiveDateTime,
}

#[post("/tags")]
async fn create_tags(
    pool: Data<DbPool>,
    request_body: Json<RequestBody>,
) -> Result<ApiResponse<ResponseData>, ApiError> {
    // First do the insert
    query!(
        "INSERT INTO tags (user_id, name) VALUES (?, ?)",
        &request_body.user_id,
        &request_body.name
    )
    .execute(pool.get_pool())
    .await?;

    // Then get the inserted row
    let tags_metadata = query_as!(
        ResponseData,
        "SELECT id, user_id, name, created_at 
        FROM tags WHERE user_id = ? 
        AND name = ? 
        ORDER BY created_at DESC LIMIT 1",
        &request_body.user_id,
        &request_body.name
    )
    .fetch_one(pool.get_pool())
    .await?;

    Ok(ApiResponse::new(
        Some(tags_metadata),
        Some("Tag created successfully".to_string()),
    ))
}
