use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use pwhash::sha512_crypt;
use rand::distributions::{Alphanumeric, DistString};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use super::DbState;

#[derive(Serialize, Deserialize)]
pub struct RegisterUser {
    username: String,
    password: String,
    email: String,
    is_male: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct IdUser {
    id: Option<Thing>,
    links: Vec<String>,
    pub password: String,
    is_male: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct Login {
    id: Thing,
}

pub async fn register(
    State(state): State<DbState>,
    Json(user): Json<RegisterUser>,
) -> impl IntoResponse {
    let hashed_password = match sha512_crypt::hash(user.password) {
        Ok(hash) => hash,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    };

    let token = Alphanumeric.sample_string(&mut rand::thread_rng(), 256);

    let id_user: IdUser = match state
        .db
        .create("user")
        .content(IdUser {
            id: None,
            links: vec![
                format!("login:{}", user.username),
                format!("login:{}", user.email),
                format!("authtoken:{token}"),
            ],
            password: hashed_password,
            is_male: user.is_male,
        })
        .await
    {
        Ok(user) => user,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    };

    let _username_user: Login = match state
        .db
        .create(("login", user.username))
        .content(Login {
            id: id_user.id.clone().unwrap(),
        })
        .await
    {
        Ok(user) => user,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    };

    let _email_user: Login = match state
        .db
        .create(("login", user.email))
        .content(Login {
            id: id_user.id.clone().unwrap(),
        })
        .await
    {
        Ok(user) => user,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    };

    let _tokens_user: Login = match state
        .db
        .create(("login", token.clone()))
        .content(Login {
            id: id_user.id.clone().unwrap(),
        })
        .await
    {
        Ok(user) => user,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    };

    let final_user: IdUser = match state.db.select(("login", token)).await {
        Ok(user) => user,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    };

    (StatusCode::OK, Json(final_user), Coo).into_response()
}
