use crate::{db::DbPool, errors::ApiError};
use actix_web::{
    body::BoxBody,
    post,
    web::{Data, Json},
    HttpRequest, HttpResponse, Responder, Result,
};
use serde::{Deserialize, Serialize};
use sqlx::{MySql, QueryBuilder};

#[derive(Deserialize, Debug)]
struct User {
    email: String,
    password: String,
}

#[derive(Deserialize, Debug)]
struct Users {
    data: Vec<User>,
}

#[derive(Serialize, Debug)]
struct Response {
    message: String,
}

impl Responder for Response {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        HttpResponse::Ok().json(self)
    }
}

#[post("/v1/users")]
async fn create_users(pool: Data<DbPool>, users: Json<Users>) -> Result<Response, ApiError> {
    let pool = pool.get_pool().await?;

    // Don't proceed if there's no data to insert
    if users.data.is_empty() {
        return Ok(Response {
            message: "No users provided".to_string(),
        });
    }

    // Create a query builder for MySQL
    let mut query_builder: QueryBuilder<MySql> =
        QueryBuilder::new("INSERT INTO users (email, password) ");

    // Start the VALUES clause
    query_builder.push_values(users.data.iter(), |mut b, user| {
        b.push_bind(&user.email).push_bind(&user.password);
    });

    // Execute the batch insert
    let result = query_builder.build().execute(&pool).await?;

    Ok(Response {
        message: format!("Successfully inserted {} users", result.rows_affected()),
    })
}
