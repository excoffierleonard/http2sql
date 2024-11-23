mod config;
mod db;
mod handlers;

use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = config::Config::from_env();

    let pool = db::create_pool(&config.database_url)
        .await
        .expect("Failed to create pool");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(handlers::test)
    })
    .bind(format!("0.0.0.0:{}", config.server_port))?
    .workers(config.workers)
    .run()
    .await
}
