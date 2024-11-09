mod error;

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use axum::{extract::State, routing::post, Json, Router};
use error::AuthError;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/refresh", post(refresh))
        .route("/challenge", post(challenge))
        .route("/update-email", post(update_email))
}

// ----------------------------- register -----------------------------

#[derive(Deserialize)]
struct RegisterRequest {
    pub email: String,
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
struct RegisterResponse {
    success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<String>,
}

// #[derive(Debug, Default)]
// struct Account {
//     id: i32,
//     email: String,
//     username: String,
//     password: String,
//     created_at: String,
//     first_name: Option<String>,
//     last_name: Option<String>,
//     date_of_birth: Option<String>,
//     phone_number: Option<String>
// }

async fn register(
    State(state): State<AppState>,
    Json(req): Json<RegisterRequest>,
) -> Result<Json<RegisterResponse>, AuthError> {
    // TODO: check if account with that email already exists

    // hash the provided password
    let password_hash = {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();

        argon2
            .hash_password(req.password.as_bytes(), &salt)
            .unwrap()
            .to_string() // FIXME: no unwrap!
    };
    let created_at = chrono::Utc::now().date_naive();

    // insert new account into the database
    sqlx::query!(
        "INSERT INTO accounts (email, username, password, created_at) VALUES ($1, $2, $3, $4)",
        req.email,
        req.username,
        password_hash,
        created_at
    )
    .execute(&state.pool)
    .await
    .map_err(|e| AuthError::Other(e.to_string()))?; // FIXME: error

    // return success response
    Ok(Json(RegisterResponse {
        success: true,
        message: None,
    }))
}

// ----------------------------- login -----------------------------

#[derive(Deserialize)]
struct LoginRequest {
    email: String,
    password: String,

    #[allow(unused)]
    remember: bool,
    #[allow(unused)]
    metadata: String,
}

#[derive(Serialize)]
struct LoginResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    refresh_token: Option<String>,
    session_token: String,
}

async fn login(
    State(state): State<AppState>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, AuthError> {
    // get password for provided email
    let password_hash =
        sqlx::query_scalar!("select password from accounts where email = $1", req.email)
            .fetch_optional(&state.pool)
            .await
            .unwrap();

    let Some(password_hash) = password_hash else {
        return Err(AuthError::WrongCredentials);
    };

    // verify password
    let parsed_hash = PasswordHash::new(&password_hash).unwrap(); // FIXME: no unwrap
    let argon = Argon2::default();

    if argon
        .verify_password(req.password.as_bytes(), &parsed_hash)
        .is_err()
    {
        return Err(AuthError::WrongCredentials);
    }

    // TODO: generate session/refresh tokens + store them in the database

    Ok(Json(LoginResponse {
        refresh_token: Some("refresh-with-this-please".into()),
        session_token: "otherwise-use-this".into(),
    }))
}

// ----------------------------- refresh session -----------------------------

#[derive(Deserialize)]
struct RefreshRequest {
    refresh_token: String,
}

#[derive(Serialize)]
struct RefreshResponse {
    refresh_token: String,
    session_token: String,
}

async fn refresh(
    State(state): State<AppState>,
    Json(req): Json<RefreshRequest>,
) -> Result<Json<RefreshResponse>, AuthError> {
    Ok(Json(RefreshResponse {
        refresh_token: "new refresh token".into(),
        session_token: req.refresh_token,
    }))
}

// ----------------------------- change email -----------------------------

#[derive(Serialize)]
struct ChangeEmailResponse {}

async fn update_email() -> Result<Json<ChangeEmailResponse>, AuthError> {
    // TODO
    Err(AuthError::NotImplemented)
}

// ----------------------------- 2fa -----------------------------
// This is for later

// ----------------------------- challenge -----------------------------

#[derive(Deserialize)]
struct ChallengeRequest {
    token: String,
    password: String,
}

#[derive(Serialize)]
struct ChallengeResponse {
    token: String,
}

async fn challenge(
    Json(req): Json<ChallengeRequest>,
) -> Result<Json<ChallengeResponse>, AuthError> {
    Ok(Json(ChallengeResponse {
        token: format!("{}, {}", req.token, req.password),
    }))
}
