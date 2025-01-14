use crate::{db::DbPool, errors::ApiError, responses::ApiResponse};
use actix_web::{get, web::Data, Result};
use serde::Serialize;
use sqlx::{query_as, types::chrono::NaiveDateTime};
use std::collections::HashMap;

#[derive(Serialize, Debug)]
struct User {
    email: String,
    created_at: NaiveDateTime,
    tags: Vec<Tag>,
}

#[derive(Serialize, Debug)]
struct Tag {
    name: String,
    created_at: NaiveDateTime,
}

#[derive(Serialize, Debug)]
struct UserRow {
    user_email: String,
    user_created_at: NaiveDateTime,
    tag_name: Option<String>,
    tag_created_at: Option<NaiveDateTime>,
}

#[get("/users")]
async fn read_user_metadata(pool: Data<DbPool>) -> Result<ApiResponse<Vec<User>>, ApiError> {
    let user_rows = query_as!(UserRow, "
        SELECT u.email as user_email, u.created_at as user_created_at, t.name as tag_name, t.created_at as tag_created_at 
        FROM users u 
        LEFT JOIN tags t 
        ON u.id = t.user_id
        ")
        .fetch_all(pool.get_pool())
        .await?;

    // Transform the flat result into the nested wanted structure
    let user_metadata = transform_rows_to_users(user_rows);

    Ok(ApiResponse::new(
        Some(user_metadata),
        Some("User metadata retrieved successfully".to_string()),
    ))
}

fn transform_rows_to_users(rows: Vec<UserRow>) -> Vec<User> {
    let mut user_map: HashMap<String, User> = HashMap::new();

    for row in rows {
        let user = user_map
            .entry(row.user_email.clone())
            .or_insert_with(|| User {
                email: row.user_email,
                created_at: row.user_created_at,
                tags: Vec::new(),
            });

        if let (Some(name), Some(created_at)) = (row.tag_name, row.tag_created_at) {
            user.tags.push(Tag { name, created_at });
        }
    }

    user_map.into_values().collect()
}
