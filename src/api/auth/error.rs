use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

pub enum AuthError {
    WrongCredentials,
    MissingCredentials,
    InvalidToken,
    NotImplemented,

    Other(String),
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthError::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials".into()),
            AuthError::MissingCredentials => {
                (StatusCode::BAD_REQUEST, "Missing credentials".into())
            }
            // AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error"),
            AuthError::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token".into()),
            AuthError::NotImplemented => (StatusCode::NOT_IMPLEMENTED, "Not implemented".into()),
            AuthError::Other(s) => (StatusCode::INTERNAL_SERVER_ERROR, s),
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}
