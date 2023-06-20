use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::{routes::DbState, structs::user::PublicUser};

pub async fn get_users(State(state): State<DbState>) -> impl IntoResponse {
    let users: Vec<PublicUser> = match state.db.select("user").await {
        Ok(users) => users,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    Json(users).into_response()
}
