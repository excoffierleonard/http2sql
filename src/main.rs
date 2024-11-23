use actix_web::{get, web, App, HttpServer, Responder};
use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::MySqlPool;

#[derive(Serialize)]
struct Response {
    message: String,
    db_timestamp: String,
}

#[get("/v1/test")]
async fn test(db: web::Data<MySqlPool>) -> impl Responder {
    // Use proper type annotation for DateTime
    let row: (DateTime<Utc>,) = sqlx::query_as("SELECT NOW()")
        .fetch_one(db.get_ref())
        .await
        .unwrap();

    let response = Response {
        message: "Test Worked".to_string(),
        db_timestamp: row.0.to_string(), // Convert DateTime to String
    };

    web::Json(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load .env file if it exists
    dotenv::dotenv().ok();

    // Get all database connection parameters from environment variables
    let db_host = std::env::var("HTTP2SQL_DB_HOST").expect("HTTP2SQL_DB_HOST must be set");
    let db_port = std::env::var("HTTP2SQL_DB_PORT").expect("HTTP2SQL_DB_PORT must be set");
    let db_name = std::env::var("HTTP2SQL_DB_NAME").expect("HTTP2SQL_DB_NAME must be set");
    let db_user = std::env::var("HTTP2SQL_DB_USER").expect("HTTP2SQL_DB_USER must be set");
    let db_password =
        std::env::var("HTTP2SQL_DB_PASSWORD").expect("HTTP2SQL_DB_PASSWORD must be set");

    // Construct the database URL
    let database_url = format!(
        "mysql://{}:{}@{}:{}/{}",
        db_user, db_password, db_host, db_port, db_name
    );

    // Create database connection pool
    let pool = MySqlPool::connect(&database_url)
        .await
        .expect("Failed to create pool");

    println!("Server starting on http://0.0.0.0:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(test)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
