mod config;
mod db;
mod handlers;

use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = config::Config::from_env();

    let lazy_pool = db::LazyPool::new(config.database_url);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(lazy_pool.clone()))
            .service(handlers::test)
    })
    .bind(format!("0.0.0.0:{}", config.server_port))?
    .workers(config.workers)
    .run()
    .await
}
