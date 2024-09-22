use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};

use crate::{
    cipher::{decrypt_password, encrypt_password},
    AppState,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/password", post(add_password))
        .route("/password/get", post(get_password))
        .route("/password/list", get(list_services))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddPasswordRequest {
    service: String,
    username: String,
    password: String,
    master_password: String,
}

pub async fn add_password(
    State(db): State<AppState>,
    Json(data): Json<AddPasswordRequest>,
) -> StatusCode {
    let encrypted_password = encrypt_password(&data.master_password, &data.password);
    db.save_password(&data.service, &data.username, &encrypted_password)
        .await
        .unwrap();
    StatusCode::OK
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetPasswordRequest {
    service: String,
    username: String,
    master_password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetPasswordResponse {
    password: String,
}

pub async fn get_password(
    State(db): State<AppState>,
    Json(data): Json<GetPasswordRequest>,
) -> Json<GetPasswordResponse> {
    tracing::debug!(form =? data);
    let encrypted_password = db
        .get_password(&data.service, &data.username)
        .await
        .unwrap();

    let password = decrypt_password(&data.master_password, &encrypted_password);

    Json(GetPasswordResponse { password })
}

pub async fn list_services(State(db): State<AppState>) -> Json<Vec<String>> {
    Json(db.list_services().await.unwrap())
}
