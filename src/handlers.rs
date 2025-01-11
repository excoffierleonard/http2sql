use crate::{db::DbPool, errors::ApiError};
use actix_web::{web::Data, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{mysql::MySqlQueryResult, query, MySql, Pool};

#[derive(Deserialize)]
pub struct CustomQueryRequest {
    pub query: String,
}

#[derive(Deserialize, Serialize)]
pub struct TableRow(pub std::collections::HashMap<String, Value>);

pub async fn get_pool(pool: Data<DbPool>) -> Result<Pool<MySql>, ApiError> {
    pool.get_pool().await.map_err(ApiError::Database)
}

pub fn validate_table_name(table_name: &str) -> Result<(), ApiError> {
    if table_name.is_empty() {
        return Err(ApiError::InvalidInput("Table name is required".to_string()));
    }
    Ok(())
}

pub async fn execute_query(
    pool: &Pool<MySql>,
    received_query: &str,
) -> Result<MySqlQueryResult, ApiError> {
    query(received_query)
        .execute(pool)
        .await
        .map_err(ApiError::Database)
}
