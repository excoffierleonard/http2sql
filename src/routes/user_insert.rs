use crate::{db::DbPool, errors::ApiError, requests::ApiRequest, responses::ApiResponse};
use actix_web::{
    post,
    web::{Data, Json},
    Result,
};
use serde::Deserialize;
use sqlx::{MySql, QueryBuilder};

#[derive(Deserialize, Debug)]
struct User {
    email: String,
    password: String,
}

#[post("/v1/users")]
async fn create_users(
    pool: Data<DbPool>,
    users: Json<ApiRequest<Vec<User>>>,
) -> Result<ApiResponse<()>, ApiError> {
    if users.data.is_empty() {
        return Ok(ApiResponse::message("No users provided"));
    }

    let mut query_builder: QueryBuilder<MySql> =
        QueryBuilder::new("INSERT INTO users (email, password) ");

    query_builder.push_values(users.data.iter(), |mut b, user| {
        b.push_bind(&user.email).push_bind(&user.password);
    });

    let result = query_builder.build().execute(pool.get_pool()).await?;

    Ok(ApiResponse::executed(
        result.rows_affected(),
        "Users created successfully",
    ))
}
