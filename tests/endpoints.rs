use actix_web::{
    test,
    web::{scope, Data},
    App,
};
use chrono::NaiveDateTime;
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
async fn register_user_success() {
    // Test-specific request types
    #[derive(Serialize, Debug)]
    struct RequestBody {
        email: String,
        password: String,
    }

    #[derive(Deserialize, Debug)]
    struct ResponseData {
        id: i32,
        email: String,
        created_at: NaiveDateTime,
    }

    // Test-specific response type
    #[derive(Deserialize, Debug)]
    struct ResponseBody {
        data: ResponseData,
        message: String,
    }

    // Setup
    let (database_url, _container) = setup_container().await;
    let pool = DbPool::new(database_url).await.unwrap();
    let app = test::init_service(
        App::new()
            .app_data(Data::new(pool))
            .service(scope("/v1").service(routes::register_user)),
    )
    .await;

    // Create request
    let request_body = RequestBody {
        email: "john.doe@gmail.com".to_string(),
        password: "Randompassword1!".to_string(),
    };
    let req = test::TestRequest::post()
        .uri("/v1/auth/register")
        .set_json(&request_body)
        .to_request();

    // Get response
    let resp = test::call_service(&app, req).await;

    // Assert the results
    assert!(resp.status().is_success());

    let response_body: ResponseBody = test::read_body_json(resp).await;
    assert_eq!(response_body.data.id, 2);
    assert_eq!(response_body.data.email, "john.doe@gmail.com");
    assert!(response_body.data.created_at.and_utc().timestamp() > 0);
    assert_eq!(
        response_body.message,
        "User registered successfully".to_string()
    );
}

#[actix_web::test]
async fn login_user_success() {
    // Test-specific request types
    #[derive(Serialize, Debug)]
    struct RequestBody {
        email: String,
        password: String,
    }

    // Test-specific response type
    #[derive(Deserialize, Debug)]
    struct ResponseBody {
        _data: Option<()>,
        message: String,
    }

    // Setup
    let (database_url, _container) = setup_container().await;
    let pool = DbPool::new(database_url).await.unwrap();
    let app = test::init_service(
        App::new()
            .app_data(Data::new(pool))
            .service(scope("/v1").service(routes::login_user)),
    )
    .await;

    // Create request
    let request_body = RequestBody {
        email: "luke.warm@hotmail.fr".to_string(),
        password: "Randompassword2!".to_string(),
    };
    let req = test::TestRequest::post()
        .uri("/v1/auth/login")
        .set_json(&request_body)
        .to_request();

    // Get response
    let resp = test::call_service(&app, req).await;

    // Assert the results
    assert!(resp.status().is_success());

    let response_body: ResponseBody = test::read_body_json(resp).await;
    assert_eq!(response_body.message, "Correct password");
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
    struct ResponseBody {
        data: Vec<ResponseUser>,
        _message: Option<()>,
    }

    // Setup
    let (database_url, _container) = setup_container().await;
    let pool = DbPool::new(database_url).await.unwrap();
    let app = test::init_service(
        App::new()
            .app_data(Data::new(pool))
            .service(scope("/v1").service(routes::custom_query)),
    )
    .await;

    // Create request
    let req = test::TestRequest::get().uri("/v1/users").to_request();

    // Get response
    let resp = test::call_service(&app, req).await;

    // Assert the results
    assert!(resp.status().is_success());

    let body: ResponseBody = test::read_body_json(resp).await;
    let users = body.data;
    assert_eq!(users.len(), 2);
    assert_eq!(users[0].id, 1);
    assert_eq!(users[0].email, "john.doe@gmail.com");
    assert_eq!(users[0].password.len(), 97);
    assert_eq!(users[1].id, 2);
    assert_eq!(users[1].email, "luke.warm@hotmail.fr");
    assert_eq!(users[1].password.len(), 97);
}
