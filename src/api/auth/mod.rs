mod error;

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use axum::{extract::State, routing::post, Json, Router};
use error::AuthError;
use serde::{Deserialize, Serialize};

use crate::{
    auth::{generate_refresh_token, generate_session_token, verify_refresh_token},
    AppState,
};

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
    struct LoginHelper {
        id: i32,
        password: String,
    }
    // get password for provided email
    let entry = sqlx::query_as!(
        LoginHelper,
        "SELECT id, password FROM accounts WHERE email = $1",
        req.email
    )
    .fetch_optional(&state.pool)
    .await
    .unwrap();

    let Some(entry) = entry else {
        return Err(AuthError::WrongCredentials);
    };

    // verify password
    let parsed_hash = PasswordHash::new(&entry.password).unwrap(); // FIXME: no unwrap
    Argon2::default()
        .verify_password(req.password.as_bytes(), &parsed_hash)
        // if there is an error, the user provided a false password
        .map_err(|_| AuthError::WrongCredentials)?;

    // short-lived session token (5 minutes)
    let session_token = generate_session_token(entry.id.to_string());
    // if remember_me == true then also generate a refresh token (expires in 7 days)
    let refresh_token = if req.remember {
        let token = generate_refresh_token(entry.id.to_string());
        sqlx::query!(
            "INSERT INTO sessions (account_id, token, expires) VALUES ($1, $2, $3)",
            entry.id,
            token,
            0
        )
        .execute(&state.pool)
        .await
        .map_err(|_| AuthError::Other("internal database error".into()))?;
        Some(token)
    } else {
        None
    };

    Ok(Json(LoginResponse {
        refresh_token,
        session_token,
    }))
}

// ----------------------------- refresh session -----------------------------

#[derive(Deserialize)]
struct RefreshRequest {
    refresh_token: String,
}

#[derive(Serialize)]
struct RefreshResponse {
    session_token: String,
}

async fn refresh(
    State(state): State<AppState>,
    Json(req): Json<RefreshRequest>,
) -> Result<Json<RefreshResponse>, AuthError> {
    // verify the token
    let _claims = verify_refresh_token(&req.refresh_token).map_err(|_| AuthError::InvalidToken)?;

    // check if the refresh token is still present in the database
    let account_id = sqlx::query_scalar!(
        "SELECT account_id FROM sessions WHERE token = $1",
        req.refresh_token
    )
    .fetch_one(&state.pool)
    .await
    .map_err(|_| AuthError::InvalidToken)?;

    let session_token = generate_session_token(account_id.to_string());

    Ok(Json(RefreshResponse { session_token }))
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
