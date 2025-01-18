# HTTP2SQL API Documentation

## Table of Contents

- [Endpoints](#endpoints)
  - [Register a new user](#register-a-new-user)
  - [Authenticate a user](#authenticate-a-user)
  - [Read all users](#read-all-users)

## Endpoints

### Register a new user

```http
POST /v1/auth/register
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
        "uuid": "3ef041b2-0c27-477c-88f4-b68ba5e8de45",
        "email": "luke.warm@hotmail.fr",
        "created_at": "2025-01-14T14:36:06"
    },
    "message": "User registered successfully"
}
```

### Authenticate a user

```http
POST /v1/auth/login
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
    "data": null,
    "message": "Correct password"
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
            "email": "john.doe@gmail.com",
            "created_at": "2025-01-14T16:22:32",
            "tags": [
                {
                    "name": "tag1",
                    "created_at": "2025-01-14T16:22:32"
                },
                {
                    "name": "tag2",
                    "created_at": "2025-01-14T16:22:32"
                }
            ]
        }
    ],
    "message": "User metadata retrieved successfully"
}
```

### Create a tag

```http
POST /v1/tags
```

#### Request Body

```json
{
    "user_uuid": "3ef041b2-0c27-477c-88f4-b68ba5e8de45",
    "name": "tag3"
}
```

#### Response Body

```json
{
    "data": {
        "uuid": "f127e4fc-db67-483f-bf9d-e0c77c1f8d9d",
        "user_uuid": "3ef041b2-0c27-477c-88f4-b68ba5e8de45",
        "name": "tag3",
        "created_at": "2025-01-15T04:59:24"
    },
    "message": "Tag created successfully"
}
```