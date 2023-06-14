use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use super::DbState;

#[derive(Serialize, Deserialize)]
pub struct PublicUser {
    username: String,
    is_male: Option<bool>,
}

pub async fn get_users(State(state): State<DbState>) -> impl IntoResponse {
    let users: Vec<PublicUser> = match state.db.select("user").await {
        Ok(users) => users,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    };

    Json(users).into_response()
}
