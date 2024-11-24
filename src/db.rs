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

    pub async fn query_fetch(
        &self,
        query: &str,
    ) -> Result<Vec<sqlx::mysql::MySqlRow>, sqlx::Error> {
        let pool = self.get_pool().await?;
        sqlx::query(query).fetch_all(&pool).await
    }

    pub async fn query_execute(
        &self,
        query: &str,
    ) -> Result<sqlx::mysql::MySqlQueryResult, sqlx::Error> {
        let pool = self.get_pool().await?;
        sqlx::query(query).execute(&pool).await
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
