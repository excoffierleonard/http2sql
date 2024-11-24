use std::env;

#[derive(Debug)]
pub struct Config {
    pub database_url: String,
    pub server_port: u16,
    pub server_workers: usize,
}

impl Config {
    pub fn build() -> Result<Self, env::VarError> {
        dotenv::dotenv().ok();

        let user = env::var("HTTP2SQL_DB_USER").unwrap_or_else(|_| "http2sql-db".to_string());
        let pass = env::var("HTTP2SQL_DB_PASS").unwrap_or_else(|_| "http2sql-db".to_string());
        let host = env::var("HTTP2SQL_DB_HOST").unwrap_or_else(|_| "http2sql-db".to_string());
        let port = env::var("HTTP2SQL_DB_PORT").unwrap_or_else(|_| "3306".to_string());
        let name = env::var("HTTP2SQL_DB_NAME").unwrap_or_else(|_| "http2sql-db".to_string());

        let server_port = env::var("HTTP2SQL_SERVER_PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse()
            .unwrap_or(8080);

        let server_workers = env::var("HTTP2SQL_SERVER_WORKERS")
            .unwrap_or_else(|_| num_cpus::get().to_string())
            .parse()
            .unwrap_or_else(|_| num_cpus::get());

        let database_url = format!("mysql://{}:{}@{}:{}/{}", user, pass, host, port, name);

        Ok(Self {
            database_url,
            server_port,
            server_workers,
        })
    }
}
