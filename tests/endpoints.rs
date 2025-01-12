use actix_web::{test, web::Data, App};
use dotenv::dotenv;
use http2sql::{db::DbPool, routes};
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::NaiveDateTime;
use std::env::var;

#[actix_web::test]
async fn read_users() {
    #[derive(Deserialize, Debug)]
    struct User {
        id: i32,
        email: String,
        password: String,
        created_at: NaiveDateTime,
    }

    #[derive(Deserialize, Debug)]
    struct Response {
        data: Vec<User>,
    }

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

    let body: Response = test::read_body_json(resp).await;
    println!("{:?}", body);
}

#[actix_web::test]
async fn create_users() {
    #[derive(Serialize, Debug)]
    struct RequestUser {
        email: String,
        password: String,
    }

    #[derive(Serialize, Debug)]
    struct RequestUsers {
        data: Vec<RequestUser>,
    }

    #[derive(Deserialize, Debug)]
    struct Response {
        message: String,
    }

    dotenv().ok();
    let database_url = var("DATABASE_URL").unwrap();
    let pool = DbPool::new(database_url);

    // Setup
    let app = test::init_service(
        App::new()
            .app_data(Data::new(pool.clone()))
            .service(routes::create_users),
    )
    .await;

    // Create request
    let request_body = RequestUsers {
        data: vec![
            RequestUser {
                email: "john.doe@gmail.com".to_string(),
                password: "randompassword1".to_string(),
            },
            RequestUser {
                email: "luke.warm@hotmail.fr".to_string(),
                password: "randompassword2".to_string(),
            },
        ],
    };
    let req = test::TestRequest::post()
        .uri("/v1/users")
        .set_json(&request_body)
        .to_request();

    // Get response
    let resp = test::call_service(&app, req).await;

    // Assert the results
    let status = resp.status();
    assert!(status.is_success());

    let response_body: Response = test::read_body_json(resp).await;
    println!("{:?}", response_body);
}
