# HTTP2SQL API Documentation

## Table of Contents

- [Endpoints](#endpoints)
  - [Create a new user](#create-a-new-user)
  - [Read all users](#read-all-users)

## Endpoints

### Create a new user

```http
POST /v1/users
```

#### Request Body

```json
{
    "data": [
        {
            "email": "john.doe@gmail.com",
            "password": "randompassword1"
        },
        {
            "email": "luke.warm@hotmail.fr",
            "password": "randompassword2"
        }
    ]
}
```

#### Response Body

```json
{
    "data": null,
    "message": "Users created successfully",
    "affected_rows": 2
}
```

### Read all users

```http
GET /v1/users
```

#### Request Body

- None

#### Response Body

```json
{
    "data": [
        {
            "id": 1,
            "email": "john.doe@gmail.com",
            "password": "randompassword1",
            "created_at": "2025-01-13T05:03:01"
        },
        {
            "id": 2,
            "email": "luke.warm@hotmail.fr",
            "password": "randompassword2",
            "created_at": "2025-01-13T05:03:01"
        }
    ],
    "message": null,
    "affected_rows": null
}
```