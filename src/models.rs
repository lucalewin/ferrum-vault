use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct PasswordEntry {
    pub id: i32,
    pub service: String,
    pub username: String,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct FullPasswordEntry {
    pub id: i32,
    pub service: String,
    pub username: String,
    pub password: String,
}
