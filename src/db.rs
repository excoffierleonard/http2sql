use log::info;
use sqlx::MySqlPool;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct DbPool {
    pool: Arc<Mutex<Option<MySqlPool>>>,
    url: String,
}

impl DbPool {
    pub fn new(url: String) -> Self {
        Self {
            pool: Arc::new(Mutex::new(None)),
            url,
        }
    }

    pub async fn get_pool(&self) -> Result<MySqlPool, sqlx::Error> {
        let mut guard = self.pool.lock().await;

        match &*guard {
            Some(pool) => Ok(pool.clone()),
            None => {
                info!("Creating new database connection");
                let pool = MySqlPool::connect(&self.url).await?;
                *guard = Some(pool.clone());
                Ok(pool)
            }
        }
    }
}

impl Clone for DbPool {
    fn clone(&self) -> Self {
        Self {
            pool: self.pool.clone(),
            url: self.url.clone(),
        }
    }
}

pub async fn get_current_timestamp(
    pool: &MySqlPool,
) -> Result<chrono::DateTime<chrono::Utc>, sqlx::Error> {
    let (timestamp,): (chrono::DateTime<chrono::Utc>,) =
        sqlx::query_as("SELECT NOW()").fetch_one(pool).await?;
    Ok(timestamp)
}
