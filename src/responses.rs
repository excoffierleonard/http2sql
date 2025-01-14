use actix_web::{body::BoxBody, HttpRequest, HttpResponse, Responder};
use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse<T> {
    data: Option<T>,
    message: Option<String>,
}

impl<T: Serialize> Responder for ApiResponse<T> {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        HttpResponse::Ok().json(self)
    }
}

impl<T> ApiResponse<T> {
    pub fn new(data: Option<T>, message: Option<String>) -> Self {
        Self { data, message }
    }
}
