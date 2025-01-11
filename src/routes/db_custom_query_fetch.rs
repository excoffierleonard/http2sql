use crate::{
    db::DbPool,
    errors::ApiError,
    handlers::{get_pool, CustomQueryRequest, TableRow},
};
use actix_web::{
    get,
    web::{Data, Json},
    HttpResponse, Responder, Result,
};
use base64::encode;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use serde_json::Value;
use sqlx::{
    mysql::{MySqlColumn, MySqlRow},
    query, Column, Row, TypeInfo,
};
use std::collections::HashMap;

#[get("/v1/custom")]
async fn custom_query_fetch(
    pool: Data<DbPool>,
    query: Json<CustomQueryRequest>,
) -> Result<impl Responder, ApiError> {
    handle_custom_query_fetch(pool, query).await
}

async fn handle_custom_query_fetch(
    pool: Data<DbPool>,
    received_query: Json<CustomQueryRequest>,
) -> Result<impl Responder, ApiError> {
    let pool = get_pool(pool).await?;

    let normalized_query = received_query.query.trim().to_uppercase();
    if !normalized_query.starts_with("SELECT") {
        return Err(ApiError::InvalidInput(
            "Only SELECT queries are allowed".to_string(),
        ));
    }

    let rows = query(&received_query.query)
        .fetch_all(&pool)
        .await
        .map_err(ApiError::Database)?;

    let result = convert_rows_to_table_rows(rows).await;
    Ok(HttpResponse::Ok().json(result))
}

async fn convert_rows_to_table_rows(rows: Vec<MySqlRow>) -> Vec<TableRow> {
    let mut result = Vec::with_capacity(rows.len());

    for row in rows {
        let mut map = HashMap::new();
        let columns = row.columns();

        for column in columns {
            let column_name = column.name();
            let value = convert_sql_value(&row, column);
            map.insert(column_name.to_string(), value);
        }

        result.push(TableRow(map));
    }

    result
}

pub fn convert_sql_value(row: &MySqlRow, column: &MySqlColumn) -> Value {
    let column_name = column.name();
    let type_name = column.type_info().name();

    match type_name {
        "TINYINT" | "SMALLINT" | "MEDIUMINT" | "INT" | "BIGINT" => {
            if type_name.contains("UNSIGNED") {
                row.try_get::<u64, _>(column_name)
                    .ok()
                    .map(Value::from)
                    .unwrap_or(Value::Null)
            } else {
                row.try_get::<i64, _>(column_name)
                    .ok()
                    .map(Value::from)
                    .unwrap_or(Value::Null)
            }
        }
        "FLOAT" | "DOUBLE" => row
            .try_get::<f64, _>(column_name)
            .ok()
            .map(Value::from)
            .unwrap_or(Value::Null),
        "DECIMAL" => row
            .try_get::<String, _>(column_name)
            .ok()
            .map(Value::from)
            .unwrap_or(Value::Null),
        "CHAR" | "VARCHAR" | "TEXT" | "TINYTEXT" | "MEDIUMTEXT" | "LONGTEXT" => row
            .try_get::<String, _>(column_name)
            .ok()
            .map(Value::from)
            .unwrap_or(Value::Null),
        "BINARY" | "VARBINARY" | "BLOB" | "TINYBLOB" | "MEDIUMBLOB" | "LONGBLOB" => row
            .try_get::<Vec<u8>, _>(column_name)
            .ok()
            .map(|bytes| Value::String(encode(&bytes)))
            .unwrap_or(Value::Null),
        "BOOL" | "BOOLEAN" => row
            .try_get::<bool, _>(column_name)
            .ok()
            .map(Value::from)
            .unwrap_or(Value::Null),
        "DATE" => row
            .try_get::<NaiveDate, _>(column_name)
            .ok()
            .map(|d| Value::String(d.format("%Y-%m-%d").to_string()))
            .unwrap_or(Value::Null),
        "TIME" => row
            .try_get::<NaiveTime, _>(column_name)
            .ok()
            .map(|t| Value::String(t.format("%H:%M:%S").to_string()))
            .unwrap_or(Value::Null),
        "DATETIME" | "TIMESTAMP" => row
            .try_get::<NaiveDateTime, _>(column_name)
            .ok()
            .map(|dt| Value::String(dt.format("%Y-%m-%d %H:%M:%S").to_string()))
            .unwrap_or(Value::Null),
        "JSON" => row
            .try_get::<Value, _>(column_name)
            .ok()
            .unwrap_or(Value::Null),
        "ENUM" | "SET" => row
            .try_get::<String, _>(column_name)
            .ok()
            .map(Value::from)
            .unwrap_or(Value::Null),
        _ => Value::Null,
    }
}
