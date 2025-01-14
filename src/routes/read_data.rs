use crate::{db::DbPool, errors::ApiError, responses::ApiResponse};
use actix_web::{get, web::Data, Result};
use serde::Serialize;
use sqlx::{query_as, types::chrono::NaiveDateTime};

#[derive(Serialize, Debug, Eq, PartialEq)]
struct User {
    email: String,
    created_at: NaiveDateTime,
    tags: Vec<Tag>,
}

#[derive(Serialize, Debug, Eq, PartialEq)]
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

// Transform the flat result into the nested wanted structure maintaining the order
fn transform_rows_to_users(rows: Vec<UserRow>) -> Vec<User> {
    let mut users = Vec::new();
    let mut current_user: Option<User> = None;

    for row in rows {
        // If we're starting a new user or this is the first user
        if current_user.is_none() || current_user.as_ref().unwrap().email != row.user_email {
            // If there was a previous user, push it to the result vector
            if let Some(user) = current_user {
                users.push(user);
            }

            // Start a new user
            current_user = Some(User {
                email: row.user_email,
                created_at: row.user_created_at,
                tags: Vec::new(),
            });
        }

        // Add tag if it exists
        if let (Some(tag_name), Some(tag_created_at)) = (row.tag_name, row.tag_created_at) {
            if let Some(user) = current_user.as_mut() {
                user.tags.push(Tag {
                    name: tag_name,
                    created_at: tag_created_at,
                });
            }
        }
    }

    // Don't forget to push the last user
    if let Some(user) = current_user {
        users.push(user);
    }

    users
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{NaiveDate, NaiveTime};

    // test the transformation
    #[test]
    fn test_transform_rows_to_users() {
        let timestamp = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2025, 1, 14).unwrap(),
            NaiveTime::from_hms_opt(19, 8, 20).unwrap(),
        );

        let input = vec![
            UserRow {
                user_email: "john.doe@gmail.com".to_string(),
                user_created_at: timestamp,
                tag_name: Some("tag1".to_string()),
                tag_created_at: Some(timestamp),
            },
            UserRow {
                user_email: "john.doe@gmail.com".to_string(),
                user_created_at: timestamp,
                tag_name: Some("tag2".to_string()),
                tag_created_at: Some(timestamp),
            },
            UserRow {
                user_email: "alice.smith@gmail.com".to_string(),
                user_created_at: timestamp,
                tag_name: Some("tag3".to_string()),
                tag_created_at: Some(timestamp),
            },
            UserRow {
                user_email: "alice.smith@gmail.com".to_string(),
                user_created_at: timestamp,
                tag_name: Some("tag4".to_string()),
                tag_created_at: Some(timestamp),
            },
        ];

        let output = transform_rows_to_users(input);

        // Assert the order of the output is the same as the input
        let expected_output = vec![
            User {
                email: "john.doe@gmail.com".to_string(),
                created_at: timestamp,
                tags: vec![
                    Tag {
                        name: "tag1".to_string(),
                        created_at: timestamp,
                    },
                    Tag {
                        name: "tag2".to_string(),
                        created_at: timestamp,
                    },
                ],
            },
            User {
                email: "alice.smith@gmail.com".to_string(),
                created_at: timestamp,
                tags: vec![
                    Tag {
                        name: "tag3".to_string(),
                        created_at: timestamp,
                    },
                    Tag {
                        name: "tag4".to_string(),
                        created_at: timestamp,
                    },
                ],
            },
        ];

        assert_eq!(output, expected_output);
    }
}
