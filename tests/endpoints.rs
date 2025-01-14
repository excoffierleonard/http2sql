use actix_web::{test, web::Data, App};
use http2sql::{db::DbPool, routes};
use serde::{Deserialize, Serialize};
use testcontainers_modules::{
    mariadb::Mariadb,
    testcontainers::{runners::AsyncRunner, ContainerAsync},
};

// Create a test container db with the predefined schema
async fn setup_container() -> (String, ContainerAsync<Mariadb>) {
    let init_sql = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/ressources/init_db.sql"
    ));
    let mariadb = Mariadb::default().with_init_sql(init_sql.to_string().into_bytes());

    let container = mariadb.start().await.unwrap();
    let database_url = format!(
        "mysql://root@{}:{}/test",
        container.get_host().await.unwrap(),
        container.get_host_port_ipv4(3306).await.unwrap()
    );

    (database_url, container)
}

#[actix_web::test]
async fn create_users() {
    // Test-specific request types
    #[derive(Serialize, Debug)]
    struct RequestUser {
        email: String,
        password: String,
    }

    #[derive(Serialize, Debug)]
    struct Request {
        data: RequestUser,
    }

    // Test-specific response type
    #[derive(Deserialize, Debug)]
    struct Response {
        data: Option<()>,
        message: Option<String>,
        affected_rows: Option<u64>,
    }

    // Setup
    let (database_url, _container) = setup_container().await;
    let pool = DbPool::new(database_url).await.unwrap();
    let app = test::init_service(
        App::new()
            .app_data(Data::new(pool))
            .service(routes::register_user),
    )
    .await;

    // Create request
    let request_body = Request {
        data: RequestUser {
            email: "john.doe@gmail.com".to_string(),
            password: "Randompassword1!".to_string(),
        },
    };
    let req = test::TestRequest::post()
        .uri("/auth/register")
        .set_json(&request_body)
        .to_request();

    // Get response
    let resp = test::call_service(&app, req).await;

    // Assert the results
    assert!(resp.status().is_success());

    let response_body: Response = test::read_body_json(resp).await;
    assert_eq!(response_body.affected_rows, Some(1));
    assert_eq!(
        response_body.message,
        Some("User registered successfully".to_string())
    );
    assert_eq!(response_body.data, None);
}

#[actix_web::test]
async fn read_users() {
    // Test-specific types
    #[derive(Deserialize, Debug)]
    struct ResponseUser {
        id: i32,
        email: String,
        password: String,
    }

    #[derive(Deserialize, Debug)]
    struct Response {
        data: Option<Vec<ResponseUser>>,
        _message: Option<String>,
        _affected_rows: Option<u64>,
    }

    // Setup
    let (database_url, _container) = setup_container().await;
    let pool = DbPool::new(database_url).await.unwrap();
    let app = test::init_service(
        App::new()
            .app_data(Data::new(pool))
            .service(routes::custom_query),
    )
    .await;

    // Create request
    let req = test::TestRequest::get().uri("/users").to_request();

    // Get response
    let resp = test::call_service(&app, req).await;

    // Assert the results
    assert!(resp.status().is_success());

    let body: Response = test::read_body_json(resp).await;
    let users = body.data.unwrap();
    assert_eq!(users.len(), 2);
    assert_eq!(users[0].id, 1);
    assert_eq!(users[0].email, "john.doe@gmail.com");
    assert_eq!(users[0].password.len(), 97);
    assert_eq!(users[1].id, 2);
    assert_eq!(users[1].email, "luke.warm@hotmail.fr");
    assert_eq!(users[1].password.len(), 97);
}
