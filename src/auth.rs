use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    RequestPartsExt,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    /// Subject
    sub: String,
    /// Expiration time
    exp: usize,
}

// FIXME: load from .env (sha256 random string)
const SESSION_SECRET_KEY: &str = "session_secret";
const REFRESH_SECRET_KEY: &str = "refresh_secret";

// ------------------ signing ------------------

/// creates a session token valid for 5 minutes
pub fn generate_session_token(sub: String) -> String {
    let expiration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize
        + 300; // 5 minutes

    let claims = Claims {
        sub,
        exp: expiration,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(SESSION_SECRET_KEY.as_ref()),
    )
    .expect("Failed to generate JWT")
}

/// creates a refresh token which is valid for 7 days
pub fn generate_refresh_token(sub: String) -> String {
    let expiration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
        + 604800; // 7 days

    let claims = Claims {
        sub,
        exp: expiration as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(REFRESH_SECRET_KEY.as_ref()),
    )
    .expect("Failed to generate JWT")
}

// ------------------ verification ------------------

pub fn verify_session_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let validation = Validation::new(Algorithm::HS256);
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(SESSION_SECRET_KEY.as_ref()),
        &validation,
    )?;
    Ok(token_data.claims)
}

pub fn verify_refresh_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let validation = Validation::new(Algorithm::HS256);
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(REFRESH_SECRET_KEY.as_ref()),
        &validation,
    )?;
    Ok(token_data.claims)
}

// ------------------ extraction ------------------

/// Wrapper for the user id. Add this if the function requires authorization
pub struct SessionUser(pub i32);

#[async_trait]
impl<S> FromRequestParts<S> for SessionUser
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        // Extract the Authorization header
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| StatusCode::UNAUTHORIZED)?;

        // Validate the token
        let claims = verify_session_token(bearer.token()).map_err(|_| StatusCode::UNAUTHORIZED)?;

        Ok(SessionUser(
            claims
                .sub
                .parse()
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
        ))
    }
}
