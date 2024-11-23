mod config;
mod db;
mod handlers;

use actix_web::{
    middleware::{Compress, Logger},
    web, App, HttpServer,
};
use log::{info, warn};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    info!("Starting HTTP2SQL service");

    let config = config::Config::from_env();
    info!("Configuration loaded");

    let lazy_pool = db::LazyPool::new(config.database_url);
    let workers = config.workers;
    info!("Starting server with {} workers", workers);

    HttpServer::new(move || {
        App::new()
            .wrap(Compress::default())
            .wrap(Logger::default())
            .app_data(web::Data::new(lazy_pool.clone()))
            .service(handlers::test)
    })
    .bind(format!("0.0.0.0:{}", config.server_port))
    .map_err(|e| {
        warn!("Failed to bind to port {}: {}", config.server_port, e);
        e
    })?
    .workers(workers)
    .run()
    .await
}
