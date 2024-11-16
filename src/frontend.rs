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
        .route("/auth/signin", get(signin_get).post(signin_post))
        .route("/htmx/passwords", get(passwords))
        .route("/htmx/passwords/view", post(view))
        .route("/htmx/signin/challenge", get(signin_challenge))

    // .route("/passwords", get(passwords))
    // .route("/passwords/view", post(view))
    // .route("/contacts", get(contacts))
    // .route("/recovery-codes", get(recovery_codes))
    // .route("/signin/challenge", get(signin_challenge))
}

// ------------ templates ------------

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {}

#[derive(Template)]
#[template(path = "passwords.html")]
pub struct PasswordsTemplate {
    pub entries: Vec<PasswordEntry>,
}

#[derive(Template)]
#[template(path = "auth/challenge.html")]
pub struct SigninChallengeTemplate {
    pub password_id: i32,
}

#[derive(Template)]
#[template(path = "auth/signin.html")]
pub struct SigninTemplate {}

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

#[derive(Debug, Serialize, Deserialize)]
pub struct SignInForm {
    pub email: String,
    pub password: String,
    pub remember: Option<bool>,
}

// ------------ routes ------------

pub async fn index() -> IndexTemplate {
    IndexTemplate {}
}

pub async fn passwords(State(db): State<AppState>) -> PasswordsTemplate {
    let entries = db.list_entries().await.unwrap();
    PasswordsTemplate { entries }
}

// FIXME
pub async fn contacts() -> IndexTemplate {
    IndexTemplate {}
}

// FIXME
pub async fn recovery_codes() -> IndexTemplate {
    IndexTemplate {}
}

pub async fn signin_get() -> SigninTemplate {
    SigninTemplate {}
}

pub async fn signin_post(Form(data): Form<SignInForm>) -> String {
    format!("{:?}", data)
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
    let password = decrypt_password(&form.master_password, &db_entry.password).unwrap();

    db_entry.password = password;

    ViewTemplate { entry: db_entry }
}
