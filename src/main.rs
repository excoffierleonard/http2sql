use actix_web::{
    middleware::{Compress, Logger},
    web::Data,
    App, HttpServer,
};
use env_logger::{init_from_env, Env};
use http2sql::{config::Config, db::DbPool, routes};
use std::io::{Error, ErrorKind, Result};

#[actix_web::main]
async fn main() -> Result<()> {
    init_from_env(Env::new().default_filter_or("info"));

    let config = Config::build().map_err(|e| Error::new(ErrorKind::Other, e))?;

    let pool = DbPool::new(config.database_url)
        .await
        .map_err(|e| Error::new(ErrorKind::Other, e))?;

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Compress::default())
            .app_data(Data::new(pool.clone()))
            .service(routes::custom_query)
            .service(routes::create_users)
    })
    .bind(format!("0.0.0.0:{}", config.server_port))?
    .workers(config.workers)
    .run()
    .await
}
