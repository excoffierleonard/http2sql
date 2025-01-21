# HTTP2SQL API Documentation

## Table of Contents

- [Endpoints](#endpoints)
  - [Register a new user](#register-a-new-user)
  - [Authenticate a user](#authenticate-a-user)

## Endpoints

### Register a new user

```http
POST /v1/auth/sign-up
```

#### Request Body

```json
{
    {
        "email": "luke.warm@hotmail.fr",
        "password": "Randompassword2!"
    }
}
```

#### Response Body

```json
{
    "data": {
        "email": "luke.warm@hotmail.fr",
        "created_at": "2025-01-14T14:36:06"
    },
    "message": "User registered successfully"
}
```

### Authenticate a user

```http
POST /v1/auth/sign-in
```

#### Request Body

```json
{
    {
        "email": "luke.warm@hotmail.fr",
        "password": "Randompassword2!"
    }
}
```

#### Response Body

```json
{
    "data": {
        "api_key": "ak_prod_IoJY0DGzXoiEqRmxr6FH/vXvHL5H26uiuGst9+3nHl0=",
        "created_at": "2025-01-14T14:36:06",
        "expires_at": "2025-01-21T14:36:06"
    },
    "message": "Password is correct, API key generated successfully"
}
```

### Fetch User Metadata

```http
GET /v1/user/{uuid}
```

#### Request Header

```http
Authorization: Bearer ak_prod_kOYoM5SeT+M3LqWdClwWZO0/E9Fogg63wGUxTuolMNQ=
```

#### Request Body

- None

#### Response Body

```json
{
    "data": {
        "uuid": "b6cea585-0dc0-4887-8247-201f164a6d6a",
        "email": "john.doe@gmail.com",
        "created_at": "2025-01-21T19:40:50"
    },
    "message": "User metadata retrieved successfully"
}
```