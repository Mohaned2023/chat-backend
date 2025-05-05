use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;


#[derive(PartialEq)]
pub enum AppError {
    ValidationError(String),
    UserFound,
    InternalServerError,
    Unauthorized,
    NotFoundUser,
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            AppError::ValidationError(e) => (StatusCode::BAD_REQUEST, e),
            AppError::UserFound => (StatusCode::FOUND, "User found!".to_string()),
            AppError::InternalServerError => (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error".to_string()),
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized".to_string()),
            AppError::NotFoundUser => (StatusCode::NOT_FOUND, "User NOT found!".to_string())
        };
        let res = Json(json!({
            "message": message,
            "status": status.as_u16()
        }));
        return (status, res).into_response();
    }
}