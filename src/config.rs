pub struct Config {
    pub database_url: String,
    pub server_port: u16,
    pub workers: usize,
}

impl Config {
    pub fn from_env() -> Result<Self, std::env::VarError> {
        dotenv::dotenv().ok();

        Ok(Self {
            database_url: format!(
                "mysql://{}:{}@{}:{}/{}",
                std::env::var("HTTP2SQL_DB_USER")?,
                std::env::var("HTTP2SQL_DB_PASSWORD")?,
                std::env::var("HTTP2SQL_DB_HOST")?,
                std::env::var("HTTP2SQL_DB_PORT")?,
                std::env::var("HTTP2SQL_DB_NAME")?
            ),
            server_port: 8080,
            workers: num_cpus::get(),
        })
    }
}
