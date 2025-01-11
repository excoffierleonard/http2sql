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
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Database(e) => write!(f, "Database error: {}", e),
            Self::InvalidInput(e) => write!(f, "Invalid input: {}", e),
        }
    }
}

impl std::error::Error for ApiError {}

impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::InvalidInput(_) => StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let error_response = ErrorResponse {
            message: self.to_string(),
        };

        HttpResponse::build(self.status_code()).json(error_response)
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

        assert_eq!(
            bad_request.to_string(),
            "Database error: no rows returned by a query that expected to return at least one row"
        );
        assert_eq!(internal_error.to_string(), "Invalid input: Wrong input");
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
    }
}
