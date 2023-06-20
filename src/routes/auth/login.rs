use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use hashes::sha3::sha512::hash;
use serde::{Deserialize, Serialize};
use tower_cookies::{Cookie, Cookies};

use crate::routes::DbState;

use super::{register::User};

#[derive(Serialize, Deserialize)]
pub struct LoginUser {
    username_or_email: String,
    password: String,
}

pub async fn login(
    cookies: Cookies,
    State(state): State<DbState>,
    Json(credentials): Json<LoginUser>,
) -> impl IntoResponse {
    let search_by = if credentials.username_or_email.contains("@") {
        "email"
    } else {
        "username"
    };

    let users = state
        .db
        .query(format!("SELECT * FROM user WHERE {search_by} = $value"))
        .bind(("value", credentials.username_or_email))
        .await;

    let mut users = match users {
        Ok(users) => users,
        Err(_) => return (StatusCode::FORBIDDEN, "Identifiants incorrects").into_response()
    };

    let option_user: Option<User> = users.take(0).unwrap_or_default();

    if let Some(user) = option_user {
        let hashed_password = hash(credentials.password.as_bytes()).into_bytes().to_vec();

        if user.password == hashed_password {
            return (
                StatusCode::OK,
                cookies.add(Cookie::new("token", user.token)),
            )
                .into_response();
        }
    }
    (StatusCode::FORBIDDEN, "Identifiants incorrects").into_response()
}
