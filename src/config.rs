use crate::errors::ApiError;
use dotenv::dotenv;
use num_cpus::get;
use std::env::var;

#[derive(Debug)]
pub struct Config {
    pub database_url: String,
    pub server_port: u16,
    pub workers: usize,
}

impl Config {
    pub fn build() -> Result<Self, ApiError> {
        dotenv().ok();

        let database_url = var("HTTP2SQL_DATABASE_URL")?;

        let server_port = var("HTTP2SQL_SERVER_PORT")
            .ok()
            .and_then(|port| port.parse().ok())
            .unwrap_or(8080);

        let workers = get();

        Ok(Self {
            database_url,
            server_port,
            workers,
        })
    }
}
