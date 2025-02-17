use actix_web::{
    test,
    web::{scope, Data},
    App,
};
use chrono::NaiveDateTime;
use http2sql::{db::DbPool, routes::v1_routes};
use serde::{Deserialize, Serialize};
use testcontainers_modules::{
    mariadb::Mariadb,
    testcontainers::{runners::AsyncRunner, ContainerAsync},
};

// Shared test utilities
mod test_utils {
    use super::*;

    pub async fn setup_container() -> (String, ContainerAsync<Mariadb>) {
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

    pub async fn setup_test_app(
        database_url: String,
    ) -> impl actix_web::dev::Service<
        actix_http::Request,
        Response = actix_web::dev::ServiceResponse,
        Error = actix_web::Error,
    > {
        let pool = DbPool::new(database_url).await.unwrap();
        test::init_service(
            App::new()
                .app_data(Data::new(pool))
                .service(scope("/v1").configure(v1_routes)),
        )
        .await
    }
}

// Shared response types
mod test_types {
    use super::*;

    #[derive(Deserialize, Debug)]
    pub struct ResponseData<T> {
        pub data: T,
        pub message: String,
    }
}

// Tests
#[actix_web::test]
async fn register_user_success() {
    #[derive(Serialize, Debug)]
    struct RequestBody {
        email: String,
        password: String,
    }

    #[derive(Deserialize, Debug)]
    struct RegisterResponse {
        uuid: String,
        email: String,
        created_at: NaiveDateTime,
    }

    let (database_url, _container) = test_utils::setup_container().await;
    let app = test_utils::setup_test_app(database_url).await;

    let request_body = RequestBody {
        email: "luke.warm@hotmail.fr".to_string(),
        password: "Randompassword2!".to_string(),
    };
    let req = test::TestRequest::post()
        .uri("/v1/auth/sign-up")
        .set_json(&request_body)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let response_body: test_types::ResponseData<RegisterResponse> =
        test::read_body_json(resp).await;
    assert_eq!(response_body.data.uuid.len(), 36);
    assert_eq!(response_body.data.email, "luke.warm@hotmail.fr");
    assert!(response_body.data.created_at.and_utc().timestamp() > 0);
    assert_eq!(response_body.message, "User registered successfully");
}

#[actix_web::test]
async fn login_user_success() {
    #[derive(Serialize, Debug)]
    struct RequestBody {
        email: String,
        password: String,
    }

    #[derive(Deserialize, Debug)]
    struct LoginResponse {
        api_key: String,
        created_at: NaiveDateTime,
        expires_at: NaiveDateTime,
    }

    let (database_url, _container) = test_utils::setup_container().await;
    let app = test_utils::setup_test_app(database_url).await;

    let request_body = RequestBody {
        email: "john.doe@gmail.com".to_string(),
        password: "Randompassword1!".to_string(),
    };
    let req = test::TestRequest::post()
        .uri("/v1/auth/sign-in")
        .set_json(&request_body)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let response_body: test_types::ResponseData<LoginResponse> = test::read_body_json(resp).await;
    assert_eq!(response_body.data.api_key.len(), 52);
    assert!(response_body.data.created_at.and_utc().timestamp() > 0);
    assert!(response_body.data.expires_at.and_utc().timestamp() > 0);
    assert_eq!(
        response_body.message,
        "Password is correct, API key generated successfully"
    );
}

#[actix_web::test]
async fn fetch_user_metadata() {
    #[derive(Deserialize, Debug)]
    struct UserMetadata {
        uuid: String,
        email: String,
        created_at: NaiveDateTime,
    }

    let (database_url, _container) = test_utils::setup_container().await;
    let app = test_utils::setup_test_app(database_url).await;

    let api_key = "ak_prod_kOYoM5SeT+M3LqWdClwWZO0/E9Fogg63wGUxTuolMNQ=";
    let req = test::TestRequest::get()
        .uri("/v1/user/metadata")
        .insert_header(("Authorization", format!("Bearer {}", api_key)))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let body: test_types::ResponseData<UserMetadata> = test::read_body_json(resp).await;
    assert_eq!(body.message, "User metadata retrieved successfully");

    let data = body.data;
    assert_eq!(data.uuid, "b6cea585-0dc0-4887-8247-201f164a6d6a");
    assert_eq!(data.email, "john.doe@gmail.com");
    assert!(data.created_at.and_utc().timestamp() > 0);
}
