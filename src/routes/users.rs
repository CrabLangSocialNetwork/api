use axum::{response::IntoResponse, Json, extract::State};

use crate::{routes::{structs::{PublicUser, ServerError}}};

use super::DbState;

pub async fn get_users(State(state): State<DbState>) -> impl IntoResponse {
    match state.db.select::<Vec<PublicUser>>("user").await {
        Ok(users) => Json(users).into_response(),
        Err(_) => return Json(ServerError{error: "Erreur lors de l'obtention des utilisateurs".to_string()}).into_response()
    }
}