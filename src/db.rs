use log::{debug, info};
use sqlx::MySqlPool;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{Duration, Instant};

// New type to represent a database pool with its timestamp
struct PoolEntry {
    pool: Arc<MySqlPool>,
    last_used: Instant,
}

#[derive(Clone)]
pub struct LazyPool {
    url: String,
    pool: Arc<Mutex<Option<PoolEntry>>>,
}

impl LazyPool {
    pub fn new(url: String) -> Self {
        info!("Initializing lazy database pool");
        Self {
            url,
            pool: Arc::new(Mutex::new(None)),
        }
    }

    pub async fn get_pool(&self) -> Result<Arc<MySqlPool>, sqlx::Error> {
        const TIMEOUT_DURATION: Duration = Duration::from_secs(300); // 5 minutes

        // First, try reading without locking
        if let Some(entry) = self.pool.lock().await.as_ref() {
            if entry.last_used.elapsed() < TIMEOUT_DURATION {
                debug!("Reusing existing database connection");
                return Ok(entry.pool.clone());
            }
        }

        // If we need a new connection, then we take the lock
        let mut guard = self.pool.lock().await;

        // Double-check pattern in case another thread created the pool
        if let Some(entry) = guard.as_ref() {
            if entry.last_used.elapsed() < TIMEOUT_DURATION {
                debug!("Reusing existing database connection (after double-check)");
                return Ok(entry.pool.clone());
            }
        }

        // Create new pool
        info!("Creating new database connection");
        let new_pool = Arc::new(MySqlPool::connect(&self.url).await?);

        *guard = Some(PoolEntry {
            pool: new_pool.clone(),
            last_used: Instant::now(),
        });

        Ok(new_pool)
    }
}

pub async fn get_current_timestamp(
    pool: &MySqlPool,
) -> Result<chrono::DateTime<chrono::Utc>, sqlx::Error> {
    debug!("Fetching current timestamp from database");
    let (timestamp,): (chrono::DateTime<chrono::Utc>,) =
        sqlx::query_as("SELECT NOW()").fetch_one(pool).await?;
    Ok(timestamp)
}
