use std::env;

pub struct Config {
    pub database_url: String,
    pub server_port: u16,
    pub workers: usize,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv::dotenv().ok();

        let db_host = env::var("HTTP2SQL_DB_HOST").expect("HTTP2SQL_DB_HOST must be set");
        let db_port = env::var("HTTP2SQL_DB_PORT").expect("HTTP2SQL_DB_PORT must be set");
        let db_name = env::var("HTTP2SQL_DB_NAME").expect("HTTP2SQL_DB_NAME must be set");
        let db_user = env::var("HTTP2SQL_DB_USER").expect("HTTP2SQL_DB_USER must be set");
        let db_password =
            env::var("HTTP2SQL_DB_PASSWORD").expect("HTTP2SQL_DB_PASSWORD must be set");

        let database_url = format!(
            "mysql://{}:{}@{}:{}/{}",
            db_user, db_password, db_host, db_port, db_name
        );

        Self {
            database_url,
            server_port: 8080,
            workers: num_cpus::get(),
        }
    }
}
