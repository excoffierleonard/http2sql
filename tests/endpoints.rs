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
    pub struct Tag {
        pub name: String,
        pub created_at: NaiveDateTime,
    }

    #[derive(Deserialize, Debug)]
    pub struct User {
        pub email: String,
        pub created_at: NaiveDateTime,
        pub tags: Vec<Tag>,
    }

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
        .uri("/v1/auth/register")
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

    let (database_url, _container) = test_utils::setup_container().await;
    let app = test_utils::setup_test_app(database_url).await;

    let request_body = RequestBody {
        email: "john.doe@gmail.com".to_string(),
        password: "Randompassword1!".to_string(),
    };
    let req = test::TestRequest::post()
        .uri("/v1/auth/login")
        .set_json(&request_body)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let response_body: test_types::ResponseData<Option<()>> = test::read_body_json(resp).await;
    assert_eq!(response_body.message, "Correct password");
}

#[actix_web::test]
async fn read_users() {
    let (database_url, _container) = test_utils::setup_container().await;
    let app = test_utils::setup_test_app(database_url).await;

    let req = test::TestRequest::get().uri("/v1/users").to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let body: test_types::ResponseData<Vec<test_types::User>> = test::read_body_json(resp).await;
    assert_eq!(body.message, "User metadata retrieved successfully");

    let users = body.data;
    assert_eq!(users.len(), 3);
    assert_eq!(users[0].email, "alice.smith@gmail.com");
    assert!(users[0].created_at.and_utc().timestamp() > 0);
    assert_eq!(users[0].tags.len(), 2);
    assert_eq!(users[0].tags[0].name, "tag1");
    assert!(users[0].tags[0].created_at.and_utc().timestamp() > 0);
    assert_eq!(users[0].tags[1].name, "tag2");
    assert!(users[0].tags[1].created_at.and_utc().timestamp() > 0);
}

#[actix_web::test]
async fn create_tags() {
    #[derive(Serialize, Debug)]
    struct RequestBody {
        user_uuid: String,
        name: String,
    }

    #[derive(Deserialize, Debug)]
    struct TagResponse {
        uuid: String,
        user_uuid: String,
        name: String,
        created_at: NaiveDateTime,
    }

    let (database_url, _container) = test_utils::setup_container().await;
    let app = test_utils::setup_test_app(database_url).await;

    let request_body = RequestBody {
        user_uuid: "b6cea585-0dc0-4887-8247-201f164a6d6a".to_string(),
        name: "tag4".to_string(),
    };
    let req = test::TestRequest::post()
        .uri("/v1/tags")
        .set_json(&request_body)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let body: test_types::ResponseData<TagResponse> = test::read_body_json(resp).await;
    assert_eq!(body.message, "Tag created successfully");

    let data = body.data;
    assert_eq!(data.uuid.len(), 36);
    assert_eq!(data.user_uuid.len(), 36);
    assert_eq!(data.name, "tag4");
    assert!(data.created_at.and_utc().timestamp() > 0);
}
