use actix_web::{test, web::Data, App};
use dotenv::dotenv;
use http2sql::{db::DbPool, routes};
use serde::Deserialize;
use sqlx::types::chrono::NaiveDateTime;
use std::env::var;

#[derive(Deserialize, Debug)]
struct User {
    id: i32,
    email: String,
    password: String,
    created_at: NaiveDateTime,
}

#[derive(Deserialize, Debug)]
struct Users {
    data: Vec<User>,
}

#[actix_web::test]
async fn get_users() {
    dotenv().ok();
    let database_url = var("DATABASE_URL").unwrap();
    let pool = DbPool::new(database_url);

    // Setup
    let app = test::init_service(
        App::new()
            .app_data(Data::new(pool.clone()))
            .service(routes::custom_query),
    )
    .await;

    // Create request
    let req = test::TestRequest::get().uri("/v1/users").to_request();

    // Get response
    let resp = test::call_service(&app, req).await;

    // Assert the results
    let status = resp.status();
    assert!(status.is_success());

    let body: Users = test::read_body_json(resp).await;
    println!("{:?}", body);
}
