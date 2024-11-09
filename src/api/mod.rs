use axum::{
    routing::{get, post},
    Router,
};

mod auth;
mod vault;

use crate::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .nest("/auth", auth::router())
        .nest("/vault", vault::router())
}
