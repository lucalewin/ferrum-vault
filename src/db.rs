use std::str::FromStr;

use serde::{Deserialize, Serialize};
use sqlx::{
    prelude::FromRow,
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
    SqlitePool,
};

use crate::{
    config::Config,
    models::{FullPasswordEntry, PasswordEntry},
};

pub struct Database {
    pool: SqlitePool,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct RowItem {
    pub username: Option<String>,
    pub password: String,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct GetRowItem {
    pub password: String,
}

impl Database {
    /// Initialize the SQLite database and create the password table
    pub async fn init(config: &Config) -> Result<Self, Box<dyn std::error::Error>> {
        tracing::debug!("connecting to database");
        let options = SqliteConnectOptions::from_str(&config.database_path)?
            .create_if_missing(true)
            .to_owned();

        let pool = SqlitePoolOptions::new().connect_with(options).await?;

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS passwords (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                service TEXT NOT NULL,
                username TEXT NOT NULL,
                password TEXT NOT NULL
            )",
        )
        .execute(&pool)
        .await
        .unwrap();

        Ok(Self { pool })
    }

    // Save an encrypted password to the database
    pub async fn save_password(
        &self,
        service: &str,
        username: &str,
        encrypted_password: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        tracing::debug!("adding new password");
        sqlx::query("INSERT INTO passwords (service, username, password) VALUES (?, ?, ?)")
            .bind(service)
            .bind(username)
            .bind(encrypted_password)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    // Retrieve an encrypted password by username from the database
    pub async fn get_password(
        &self,
        service: &str,
        username: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        tracing::debug!("getting password: {}", username);
        let row: GetRowItem =
            sqlx::query_as("SELECT password FROM passwords WHERE service = ? AND username = ?")
                .bind(service)
                .bind(username)
                .fetch_one(&self.pool)
                .await?;

        Ok(row.password)
    }

    pub async fn get_password_by_id(
        &self,
        id: i32,
    ) -> Result<FullPasswordEntry, Box<dyn std::error::Error>> {
        tracing::debug!("getting password by id: {}", id);
        let row: FullPasswordEntry = sqlx::query_as("SELECT * FROM passwords WHERE id = ?")
            .bind(id)
            .fetch_one(&self.pool)
            .await?;

        Ok(row)
    }

    pub async fn list_services(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let services: Vec<ServiceRow> = sqlx::query_as("SELECT DISTINCT service FROM passwords")
            .fetch_all(&self.pool)
            .await?;

        Ok(services.into_iter().map(|r| r.service).collect())
    }

    pub async fn list_entries(&self) -> Result<Vec<PasswordEntry>, Box<dyn std::error::Error>> {
        let entries: Vec<PasswordEntry> =
            sqlx::query_as("SELECT id, service, username FROM passwords")
                .fetch_all(&self.pool)
                .await?;

        Ok(entries)
    }
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct ServiceRow {
    pub service: String,
}
