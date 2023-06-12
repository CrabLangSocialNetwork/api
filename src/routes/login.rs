use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use super::{register::IdUser, DbState};

#[derive(Serialize, Deserialize)]
pub struct PublicUser {
    username: String,
    is_male: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct LoginUser {
    username_or_email: String,
    password: String,
}

pub async fn login(State(state): State<DbState>, Json(user): Json<LoginUser>) -> impl IntoResponse {
    let fetched_user: IdUser = match state
        .db
        .select(("login", user.username_or_email.clone()))
        .await
    {
        Ok(user) => user,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json("Erreur lors de l'obtention des utilisateurs"),
            )
                .into_response()
        }
    };

    if user.password == fetched_user.password {
        Json(fetched_user).into_response()
    } else {
        StatusCode::FORBIDDEN.into_response()
    }
}
