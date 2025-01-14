use actix_web::{body::BoxBody, HttpRequest, HttpResponse, Responder};
use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub data: Option<T>,
    pub message: Option<String>,
    pub affected_rows: Option<u64>,
}

impl<T: Serialize> Responder for ApiResponse<T> {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        HttpResponse::Ok().json(self)
    }
}

impl<T> ApiResponse<T> {
    pub fn data(data: T) -> Self {
        Self {
            data: Some(data),
            message: None,
            affected_rows: None,
        }
    }

    pub fn _message(message: impl Into<String>) -> Self {
        Self {
            data: None,
            message: Some(message.into()),
            affected_rows: Some(0),
        }
    }

    pub fn executed(affected_rows: u64, message: impl Into<String>) -> Self {
        Self {
            data: None,
            message: Some(message.into()),
            affected_rows: Some(affected_rows),
        }
    }
}
