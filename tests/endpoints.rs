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
    // Setup test container for db
    let (database_url, _container) = setup_container().await;

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
async fn read_users() {
    // Setup test container for db
    let (database_url, _container) = setup_container().await;

    #[derive(Deserialize, Debug)]
    struct User {
        id: i32,
        email: String,
        password: String,
    }

    #[derive(Deserialize, Debug)]
    struct Response {
        data: Vec<User>,
    }

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
