use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use serde::Serialize;

/// The response body for an error response
#[derive(Serialize)]
pub struct ErrorResponse {
    /// The error message
    pub message: String,
}

#[derive(Debug)]
pub enum ApiError {
    Database(sqlx::Error),
    InvalidInput(String),
    ConfigError(String),
    HashError(argon2::password_hash::Error),
    Unauthorized(String),
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Database(e) => write!(f, "Database error: {}", e),
            Self::InvalidInput(e) => write!(f, "Invalid input: {}", e),
            Self::ConfigError(e) => write!(f, "Config error: {}", e),
            Self::HashError(e) => write!(f, "Hash error: {}", e),
            Self::Unauthorized(e) => write!(f, "Unauthorized: {}", e),
        }
    }
}

impl std::error::Error for ApiError {}

impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::InvalidInput(_) => StatusCode::BAD_REQUEST,
            Self::ConfigError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::HashError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::Unauthorized(_) => StatusCode::UNAUTHORIZED,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let error_response = ErrorResponse {
            message: self.to_string(),
        };

        HttpResponse::build(self.status_code()).json(error_response)
    }
}

impl From<std::env::VarError> for ApiError {
    fn from(err: std::env::VarError) -> Self {
        ApiError::ConfigError(err.to_string())
    }
}

impl From<sqlx::Error> for ApiError {
    fn from(err: sqlx::Error) -> Self {
        ApiError::Database(err)
    }
}

impl From<argon2::password_hash::Error> for ApiError {
    fn from(err: argon2::password_hash::Error) -> Self {
        ApiError::HashError(err)
    }
}

impl From<base64::DecodeError> for ApiError {
    fn from(_: base64::DecodeError) -> Self {
        ApiError::InvalidInput("Invalid base64 format".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        // Test each error variant's Display implementation
        let bad_request = ApiError::Database(sqlx::Error::RowNotFound);
        let internal_error = ApiError::InvalidInput("Wrong input".to_string());
        let config_error = ApiError::ConfigError("Missing environment variable".to_string());
        let hash_error = ApiError::HashError(argon2::password_hash::Error::Algorithm);
        let unauthorized = ApiError::Unauthorized("Unauthorized".to_string());

        assert_eq!(
            bad_request.to_string(),
            "Database error: no rows returned by a query that expected to return at least one row"
        );
        assert_eq!(internal_error.to_string(), "Invalid input: Wrong input");
        assert_eq!(
            config_error.to_string(),
            "Config error: Missing environment variable"
        );
        assert_eq!(hash_error.to_string(), "Hash error: unsupported algorithm");
        assert_eq!(unauthorized.to_string(), "Unauthorized: Unauthorized");
    }

    #[test]
    fn test_status_codes() {
        // Test each error variant's status code
        assert_eq!(
            ApiError::Database(sqlx::Error::RowNotFound).status_code(),
            StatusCode::INTERNAL_SERVER_ERROR
        );
        assert_eq!(
            ApiError::InvalidInput("Wrong input".to_string()).status_code(),
            StatusCode::BAD_REQUEST
        );
        assert_eq!(
            ApiError::ConfigError("Missing environment variable".to_string()).status_code(),
            StatusCode::INTERNAL_SERVER_ERROR
        );
        assert_eq!(
            ApiError::HashError(argon2::password_hash::Error::Algorithm).status_code(),
            StatusCode::INTERNAL_SERVER_ERROR
        );
        assert_eq!(
            ApiError::Unauthorized("Unauthorized".to_string()).status_code(),
            StatusCode::UNAUTHORIZED
        );
    }
}
