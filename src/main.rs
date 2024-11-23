use actix_web::{get, web, App, HttpServer, Responder};
use serde::Serialize;

#[derive(Serialize)]
struct Response {
    message: String,
}

#[get("/v1/test")]
async fn test() -> impl Responder {
    let response = Response {
        message: "Test Worked".to_string(),
    };
    web::Json(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server starting on http://0.0.0.0:8080");

    HttpServer::new(|| App::new().service(test))
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
