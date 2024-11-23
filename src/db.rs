use sqlx::MySqlPool;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{Duration, Instant};

#[derive(Clone)]
pub struct LazyPool {
    url: String,
    pool: Arc<Mutex<Option<(MySqlPool, Instant)>>>,
}

impl LazyPool {
    pub fn new(url: String) -> Self {
        Self {
            url,
            pool: Arc::new(Mutex::new(None)),
        }
    }

    pub async fn get_pool(&self) -> Result<MySqlPool, sqlx::Error> {
        const TIMEOUT_DURATION: Duration = Duration::from_secs(300); // 5 minutes

        let mut guard = self.pool.lock().await;

        let pool_entry = guard.take();

        let new_pool = match pool_entry {
            Some((pool, last_used)) => {
                if last_used.elapsed() < TIMEOUT_DURATION {
                    // Reset timer and return existing pool
                    pool
                } else {
                    // Connection expired, create new one
                    MySqlPool::connect(&self.url).await?
                }
            }
            None => {
                // First connection
                MySqlPool::connect(&self.url).await?
            }
        };

        // Store the new/existing pool with current timestamp
        *guard = Some((new_pool.clone(), Instant::now()));

        Ok(new_pool)
    }
}

pub async fn get_current_timestamp(
    pool: &MySqlPool,
) -> Result<chrono::DateTime<chrono::Utc>, sqlx::Error> {
    let (timestamp,): (chrono::DateTime<chrono::Utc>,) =
        sqlx::query_as("SELECT NOW()").fetch_one(pool).await?;
    Ok(timestamp)
}
