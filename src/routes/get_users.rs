use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Datetime;

use super::DbState;

#[derive(Serialize, Deserialize)]
pub struct PublicUser {
    username: String,
    is_male: Option<bool>,
    created_at: Datetime,
    updated_at: Datetime
}

pub async fn get_users(State(state): State<DbState>) -> impl IntoResponse {
    let users: Vec<PublicUser> = match state.db.select("user").await {
        Ok(users) => users,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    Json(users).into_response()
}
