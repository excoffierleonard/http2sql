mod config;
mod db;
mod handlers;

use actix_web::{
    middleware::{Compress, Logger},
    web, App, HttpServer,
};
use log::{error, info};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let config = config::Config::from_env().map_err(|e| {
        error!("Configuration error: {}", e);
        std::io::Error::new(std::io::ErrorKind::Other, e)
    })?;

    let pool = db::DbPool::new(config.database_url);

    info!("Starting server with {} workers", config.server_workers);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Compress::default())
            .app_data(web::Data::new(pool.clone()))
            .service(handlers::test)
    })
    .bind(format!("0.0.0.0:{}", config.server_port))?
    .workers(config.server_workers)
    .run()
    .await
}
