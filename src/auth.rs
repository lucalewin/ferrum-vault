use axum::{async_trait, extract::FromRequestParts, http::{request::Parts, StatusCode}};

use crate::AppState;

pub struct SessionUser {
    pub id: i32,
}

#[async_trait]
impl FromRequestParts<AppState> for SessionUser {
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, Self::Rejection> {
        let auth = parts
            .headers
            .get("Authorization")
            .ok_or(StatusCode::UNAUTHORIZED)?
            .to_str()
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        let token = sqlx::query_as!(SessionUser, "SELECT account_id AS id FROM sessions WHERE token = $1", auth)
            .fetch_optional(&state.pool)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        token.ok_or(StatusCode::UNAUTHORIZED)
    }
}
