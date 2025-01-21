use crate::{
    db::DbPool, errors::ApiError, middleware::api_key::auth_to_uuid, responses::ApiResponse,
};
use actix_web::{get, web::Data, Result};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use serde::Serialize;
use sqlx::{query_as, types::chrono::NaiveDateTime};

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
    // Authenticate the user using the API key and get the user_uuid
    let uuid = auth_to_uuid(&pool, auth.token()).await?;

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
