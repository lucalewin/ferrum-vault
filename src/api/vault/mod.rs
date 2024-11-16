use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json, Router,
};
use serde_json::json;

mod contacts;
mod passwords;
mod recovery_codes;

use crate::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .nest("/contacts", contacts::router())
        .nest("/passwords", passwords::router())
        .nest("/recovery-codes", recovery_codes::router())
}

pub enum VaultError {
    EncryptionError,
    DatabaseError,
    EntryNotFound,
    Other(String),
}

impl IntoResponse for VaultError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            VaultError::DatabaseError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal database error".into(),
            ),
            VaultError::EntryNotFound => (StatusCode::NOT_FOUND, "entry not found".into()),
            VaultError::EncryptionError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "An encryption error occured".into(),
            ),
            VaultError::Other(s) => (StatusCode::INTERNAL_SERVER_ERROR, s),
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}
