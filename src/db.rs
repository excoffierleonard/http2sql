use sqlx::MySqlPool;

pub async fn create_pool(database_url: &str) -> Result<MySqlPool, sqlx::Error> {
    MySqlPool::connect(database_url).await
}

pub async fn get_current_timestamp(
    pool: &MySqlPool,
) -> Result<chrono::DateTime<chrono::Utc>, sqlx::Error> {
    let (timestamp,): (chrono::DateTime<chrono::Utc>,) =
        sqlx::query_as("SELECT NOW()").fetch_one(pool).await?;
    Ok(timestamp)
}
