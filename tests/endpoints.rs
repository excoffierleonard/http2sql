use actix_web::{test, web::Data, App};
use dotenv::dotenv;
use http2sql::{db::DbPool, routes};
use serde::{Deserialize, Serialize};
use serial_test::serial;
use sqlx::{query, types::chrono::NaiveDateTime};
use std::env::var;

// Logic to create test container will go here

async fn setup_db() {
    dotenv().ok();
    let database_url = var("DATABASE_URL").unwrap();
    let pool = DbPool::new(database_url).get_pool().await.unwrap();

    query("DROP TABLE IF EXISTS tags")
        .execute(&pool)
        .await
        .unwrap();

    query("DROP TABLE IF EXISTS users")
        .execute(&pool)
        .await
        .unwrap();

    query(
        "CREATE TABLE users (
            `id` INT NOT NULL AUTO_INCREMENT,
            `email` VARCHAR(255) NOT NULL,
            `password` VARCHAR(255) NOT NULL,
            `created_at` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
            PRIMARY KEY (`id`)
    )",
    )
    .execute(&pool)
    .await
    .unwrap();

    query(
        "CREATE TABLE tags (
            `id` INT NOT NULL AUTO_INCREMENT,
            `user_id` INT NOT NULL,
            `name` VARCHAR(255) NOT NULL,
            `created_at` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
            PRIMARY KEY (`id`),
            FOREIGN KEY (`user_id`) REFERENCES `users`(`id`)
    )",
    )
    .execute(&pool)
    .await
    .unwrap();

    query(
        "INSERT INTO `users` (`email`, `password`) 
        VALUES ('john.doe@gmail.com', 'randompassword1'), 
               ('luke.warm@hotmail.fr', 'randompassword2')",
    )
    .execute(&pool)
    .await
    .unwrap();

    query(
        "INSERT INTO `tags` (`user_id`, `name`) 
        VALUES (1, 'tag1'), 
               (1, 'tag2'), 
               (2, 'tag3')",
    )
    .execute(&pool)
    .await
    .unwrap();
}

#[actix_web::test]
#[serial]
async fn create_users() {
    setup_db().await;

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
    assert_eq!(response_body.message, "Successfully created 2 users");
}

#[actix_web::test]
#[serial]
async fn read_users() {
    setup_db().await;

    #[derive(Deserialize, Debug)]
    struct User {
        id: i32,
        email: String,
        password: String,
        // created_at: NaiveDateTime,
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
    assert_eq!(body.data.len(), 2);
    assert_eq!(body.data[0].id, 1);
    assert_eq!(body.data[0].email, "john.doe@gmail.com");
    assert_eq!(body.data[0].password, "randompassword1");
    assert_eq!(body.data[1].id, 2);
    assert_eq!(body.data[1].email, "luke.warm@hotmail.fr");
    assert_eq!(body.data[1].password, "randompassword2");
}
