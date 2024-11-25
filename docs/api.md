# HTTP2SQL API Documentation

This API provides endpoints for managing database tables and executing custom queries. All endpoints are prefixed with `/v1`.

## Table of Contents
1. [Endpoints](#endpoints)
   - [Create Table](#create-table)
   - [Insert Rows](#insert-rows)
   - [Delete Table](#delete-table)
   - [Execute Custom Query (SELECT)](#execute-custom-query-select)
   - [Execute Custom Query (Non-SELECT)](#execute-custom-query-non-select)
2. [Data Types](#data-types)
   - [Numeric Types](#numeric-types)
   - [String Types](#string-types)
   - [Binary Types](#binary-types)
   - [Date and Time Types](#date-and-time-types)
   - [Other Types](#other-types)
3. [Error Responses](#error-responses)

## Endpoints

### Create Table
Creates a new table in the database.

**Endpoint:** `POST /v1/tables`

**Request Body:**
```json
{
  "table_name": "string",
  "columns": [
    {
      "name": "string",
      "data_type": "string",
      "constraints": ["string"] // Optional
    }
  ]
}
```

**Example Request:**
```bash
curl -X POST http://localhost:8080/v1/tables \
  -H "Content-Type: application/json" \
  -d '{
    "table_name": "users",
    "columns": [
      {
        "name": "id",
        "data_type": "INT",
        "constraints": ["PRIMARY KEY", "AUTO_INCREMENT"]
      },
      {
        "name": "username",
        "data_type": "VARCHAR(255)",
        "constraints": ["NOT NULL", "UNIQUE"]
      },
      {
        "name": "created_at",
        "data_type": "DATETIME",
        "constraints": ["DEFAULT CURRENT_TIMESTAMP"]
      }
    ]
  }'
```

**Response Codes:**
- `201 Created`: Table created successfully
- `400 Bad Request`: Invalid input (empty table name or no columns)
- `500 Internal Server Error`: Database error

### Insert Rows
Inserts one or more rows into a specified table.

**Endpoint:** `POST /v1/tables/{table_name}/rows`

**Request Body:**
```json
[
  {
    "column_name": "value"
  }
]
```

**Example Request:**
```bash
curl -X POST http://localhost:8080/v1/tables/users/rows \
  -H "Content-Type: application/json" \
  -d '[
    {
      "username": "john_doe",
      "email": "john@example.com"
    },
    {
      "username": "jane_doe",
      "email": "jane@example.com"
    }
  ]'
```

**Response Codes:**
- `201 Created`: Rows inserted successfully
- `400 Bad Request`: Invalid input (empty table name or no rows)
- `500 Internal Server Error`: Database error

### Delete Table
Deletes a table from the database.

**Endpoint:** `DELETE /v1/tables/{table_name}`

**Example Request:**
```bash
curl -X DELETE http://localhost:8080/v1/tables/users
```

**Response Codes:**
- `204 No Content`: Table deleted successfully
- `400 Bad Request`: Invalid input (empty table name)
- `500 Internal Server Error`: Database error

### Execute Custom Query (SELECT)
Executes a custom SELECT query and returns the results.

**Endpoint:** `GET /v1/custom`

**Request Body:**
```json
{
  "query": "string"
}
```

**Example Request:**
```bash
curl -X GET http://localhost:8080/v1/custom \
  -H "Content-Type: application/json" \
  -d '{
    "query": "SELECT * FROM users WHERE username LIKE '\''john%'\''"
  }'
```

**Example Response:**
```json
[
  {
    "id": 1,
    "username": "john_doe",
    "email": "john@example.com",
    "created_at": "2024-01-01 12:00:00"
  }
]
```

**Response Codes:**
- `200 OK`: Query executed successfully
- `400 Bad Request`: Invalid input (non-SELECT query)
- `500 Internal Server Error`: Database error

### Execute Custom Query (Non-SELECT)
Executes a custom non-SELECT query (INSERT, UPDATE, DELETE, etc.).

**Endpoint:** `POST /v1/custom`

**Request Body:**
```json
{
  "query": "string"
}
```

**Example Request:**
```bash
curl -X POST http://localhost:8080/v1/custom \
  -H "Content-Type: application/json" \
  -d '{
    "query": "UPDATE users SET email = '\''newemail@example.com'\'' WHERE username = '\''john_doe'\''"
  }'
```

**Response Codes:**
- `200 OK`: Query executed successfully (for UPDATE, DELETE)
- `201 Created`: Query executed successfully (for INSERT, CREATE)
- `400 Bad Request`: Invalid input (SELECT query)
- `500 Internal Server Error`: Database error

## Data Types
The API supports the following MySQL data types:

### Numeric Types
- `TINYINT`, `SMALLINT`, `MEDIUMINT`, `INT`, `BIGINT` (signed and unsigned)
- `FLOAT`, `DOUBLE`
- `DECIMAL`

### String Types
- `CHAR`, `VARCHAR`
- `TEXT`, `TINYTEXT`, `MEDIUMTEXT`, `LONGTEXT`

### Binary Types
- `BINARY`, `VARBINARY`
- `BLOB`, `TINYBLOB`, `MEDIUMBLOB`, `LONGBLOB`

### Date and Time Types
- `DATE` (format: "YYYY-MM-DD")
- `TIME` (format: "HH:MM:SS")
- `DATETIME`, `TIMESTAMP` (format: "YYYY-MM-DD HH:MM:SS")

### Other Types
- `BOOL`, `BOOLEAN`
- `JSON`
- `ENUM`, `SET`

## Error Responses
All error responses follow this format:

```json
{
  "message": "string"
}
```

Example error response:
```json
{
  "message": "Database error: duplicate key value violates unique constraint"
}
```