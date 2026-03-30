use axum::{http::StatusCode, response::{IntoResponse, Response}, Json};
use serde::Serialize;

#[derive(Serialize)]
pub struct DinamicResponse<T: Serialize> {
    pub success: bool,
    pub message: String,
    pub data: Option<T>,
}

impl<T: Serialize> DinamicResponse<T> {
    pub fn success(message: impl Into<String>, data: T) -> Self {
        Self {
            success: true,
            message: message.into(),
            data: Some(data),
        }
    }

    pub fn error(message: impl Into<String>) -> DinamicResponse<()> {
        DinamicResponse{
            success: false,
            message: message.into(),
            data: None,
        }
    }
}

/// Wrapper para retornar responsta com status HTTP
pub struct ApiResponse<T: Serialize>(pub StatusCode, pub DinamicResponse<T>);

impl<T: Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> Response {
        (self.0, Json(self.1)).into_response()
    }
}