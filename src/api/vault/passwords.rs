use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};

use crate::{
    auth::SessionUser,
    cipher::{decrypt_password, encrypt_password},
    AppState,
};

use super::VaultError;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(get_all_entries).post(create_entry))
        .route(
            "/:id",
            get(get_entry).put(update_entry).delete(delete_entry),
        )
        .route("/import", post(import_entries))
        .route("/export", get(export_entries))
}

// ---------------------------------------------------------------------------

#[derive(Deserialize)]
struct CreateRequest {
    passphrase: String,
    name: String,
    email: String,
    password: String,
    url: String,
    note: Option<String>,
}

#[derive(Serialize)]
struct CreateResponse {
    success: bool,
}

async fn create_entry(
    session: SessionUser,
    State(state): State<AppState>,
    Json(req): Json<CreateRequest>,
) -> Result<Json<CreateResponse>, VaultError> {
    tracing::debug!("create new entry");

    // TODO: verify passphrase

    let encrypted = encrypt_password(&req.passphrase, &req.password).map_err(|_| VaultError::EncryptionError)?;

    sqlx::query!(
        "INSERT INTO passwords (account_id, name, email, password, url, note) VALUES ($1, $2, $3, $4, $5, $6)",
        session.0,
        req.name,
        req.email,
        encrypted,
        req.url,
        req.note,
    ).execute(&state.pool).await.unwrap();

    Ok(Json(CreateResponse { success: true }))
}

// ---------------------------------------------------------------------------

#[derive(Serialize)]
struct PasswordEntry {
    id: i32,
    name: String,
    email: String,
    url: String,
    note: Option<String>,
}

async fn get_all_entries(
    State(state): State<AppState>,
) -> Result<Json<Vec<PasswordEntry>>, VaultError> {
    tracing::debug!("getting all entries");

    let account_id = 1; // FIXME

    let entries = sqlx::query_as!(
        PasswordEntry,
        "SELECT id, name, email, url, note FROM passwords WHERE account_id = $1",
        account_id
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|_| VaultError::DatabaseError)?;

    Ok(Json(entries))
}

// ---------------------------------------------------------------------------

#[derive(Deserialize)]
struct GetEntryRequest {
    passphrase: String,
    challenge_token: String,
}

#[derive(Serialize)]
struct FullPasswordEntry {
    id: i32,
    name: String,
    email: String,
    password: String,
    url: String,
    note: Option<String>,
}

async fn get_entry(
    Path(entry_id): Path<i32>,
    session: SessionUser,
    State(state): State<AppState>,
    Json(req): Json<GetEntryRequest>,
) -> Result<Json<FullPasswordEntry>, VaultError> {
    tracing::trace!("getting entry `{}`", entry_id);

    // TODO: verify challenge token + passphrase

    let mut entry = sqlx::query_as!(
        FullPasswordEntry,
        "SELECT id, name, email, password, url, note FROM passwords WHERE id = $1 AND account_id = $2",
        entry_id,
        session.0
    )
    .fetch_optional(&state.pool)
    .await
    .map_err(|_| VaultError::DatabaseError)?
    .ok_or(VaultError::EntryNotFound)?;

    entry.password = decrypt_password(&req.passphrase, &entry.password)
        .map_err(|_| VaultError::EncryptionError)?;

    Ok(Json(entry))
}

async fn update_entry(Path(id): Path<i32>) {
    tracing::trace!("updating entry `{}`", id);
}

async fn delete_entry(Path(id): Path<i32>) {
    tracing::trace!("deleting entry `{}`", id);
}

async fn import_entries() {
    tracing::trace!("importing entries");
}

async fn export_entries() {
    tracing::trace!("exporting entries");
}
