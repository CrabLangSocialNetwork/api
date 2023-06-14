use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use pwhash::sha512_crypt;
use rand::distributions::{Alphanumeric, DistString};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use super::{get_users::PublicUser, DbState};

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
    password: String,
    is_male: Option<bool>,
    token: String,
}

pub async fn register(
    State(state): State<DbState>,
    Json(register_user): Json<RegisterUser>,
) -> impl IntoResponse {
    let token = Alphanumeric.sample_string(&mut rand::thread_rng(), 256);

    let hashed_password = match sha512_crypt::hash(register_user.password) {
        Ok(hash) => hash,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    };

    let user: PublicUser = match state
        .db
        .create("user")
        .content(User {
            id: None,
            email: register_user.email,
            username: register_user.username,
            password: hashed_password,
            is_male: register_user.is_male,
            token,
        })
        .await
    {
        Ok(user) => user,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    };

    (StatusCode::CREATED, Json(user)).into_response()
}
