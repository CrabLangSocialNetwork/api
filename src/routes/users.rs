use axum::{response::IntoResponse, Json};

use crate::routes::{DB, structs::{PublicUser, ServerError}};

pub async fn get_users() -> impl IntoResponse {
    match DB.select::<Vec<PublicUser>>("user").await {
        Ok(users) => Json(users).into_response(),
        Err(_) => return Json(ServerError{error: "Erreur lors de l'obtention des utilisateurs".to_string()}).into_response()
    }
}