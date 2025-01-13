use crate::errors::ApiError;
use sqlx::MySqlPool;

#[derive(Clone)]
pub struct DbPool {
    pool: MySqlPool,
}

impl DbPool {
    pub async fn new(url: String) -> Result<Self, ApiError> {
        let pool = MySqlPool::connect(&url).await?;
        Ok(Self { pool })
    }

    pub fn get_pool(&self) -> &MySqlPool {
        &self.pool
    }
}
