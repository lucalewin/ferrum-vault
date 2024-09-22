use askama::Template;
use axum::{
    extract::{Query, State},
    routing::{get, post},
    Form, Router,
};
use serde::{Deserialize, Serialize};

use crate::{
    cipher::decrypt_password,
    models::{FullPasswordEntry, PasswordEntry},
    AppState,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(index))
        .route("/signin/challenge", get(signin_challenge))
        .route("/view", post(view))
}

// ------------ templates ------------

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub entries: Vec<PasswordEntry>,
}

#[derive(Template)]
#[template(path = "auth/challenge.html")]
pub struct SigninChallengeTemplate {
    pub password_id: i32,
}

#[derive(Template)]
#[template(path = "view.html")]
pub struct ViewTemplate {
    pub entry: FullPasswordEntry,
}

// ------------ params / forms ------------

#[derive(Debug, Serialize, Deserialize)]
pub struct SigninChallengeParams {
    pub id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ViewRequestForm {
    pub id: i32,
    pub master_password: String,
}

// ------------ routes ------------

pub async fn index(State(db): State<AppState>) -> IndexTemplate {
    let entries = db.list_entries().await.unwrap();
    IndexTemplate { entries }
}

pub async fn signin_challenge(
    Query(params): Query<SigninChallengeParams>,
) -> SigninChallengeTemplate {
    SigninChallengeTemplate {
        password_id: params.id,
    }
}

pub async fn view(State(db): State<AppState>, Form(form): Form<ViewRequestForm>) -> ViewTemplate {
    let mut db_entry = db.get_password_by_id(form.id).await.unwrap();
    let password = decrypt_password(&form.master_password, &db_entry.password);

    db_entry.password = password;

    ViewTemplate { entry: db_entry }
}
