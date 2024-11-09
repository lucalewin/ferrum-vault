use std::sync::Arc;

use axum::Router;
use config::Config;
use db::Database;
use tokio::net::TcpListener;
use tower_http::{services::ServeDir, trace::TraceLayer};

mod api;
mod cipher;
mod config;
mod db;
mod frontend;
mod models;
pub mod auth;

pub type AppState = Arc<Database>;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .init();

    let config = Config::load();

    let db = Database::init(&config).await.unwrap();

    let app = Router::new()
        .nest("/api/v1", api::router())
        .nest("/", frontend::router())
        .nest_service(
            "/assets",
            ServeDir::new("./assets").append_index_html_on_directories(false),
        )
        .layer(TraceLayer::new_for_http())
        .with_state(Arc::new(db));

    let listener = TcpListener::bind((config.address, config.port))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap()
}
