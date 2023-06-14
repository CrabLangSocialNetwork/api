use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use hashes::sha3::sha512::hash;
use rand::distributions::{Alphanumeric, DistString};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
use tower_cookies::{Cookie, Cookies};

use super::DbState;

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterUser {
    email: String,
    username: String,
    password: String,
    is_male: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    id: Option<Thing>,
    email: String,
    username: String,
    pub(crate) password: Vec<u8>,
    is_male: Option<bool>,
    pub(crate) token: String,
}

pub async fn register(
    cookies: Cookies,
    State(state): State<DbState>,
    Json(register_user): Json<RegisterUser>,
) -> impl IntoResponse {
    let token = Alphanumeric.sample_string(&mut rand::thread_rng(), 256);

    let hashed_password = hash(register_user.password.as_bytes())
        .into_bytes()
        .to_vec();

    match state
        .db
        .create("user")
        .content(User {
            id: None,
            email: register_user.email,
            username: register_user.username,
            password: hashed_password,
            is_male: register_user.is_male,
            token: token.clone(),
        })
        .await
    {
        Ok(user) => user,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    };

    (
        StatusCode::CREATED,
        cookies.add(Cookie::new("token", token)),
    )
        .into_response()
}
