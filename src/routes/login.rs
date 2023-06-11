use axum::{Json, response::IntoResponse, http::StatusCode, extract::State};
use serde::{Deserialize, Serialize};


use crate::{routes::{structs::{ServerError, ServerSuccess, User}}};

use super::DbState;

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginUser {
    username_or_email: String,
    password: String
}

pub async fn login(State(state): State<DbState>, Json(user): Json<LoginUser>) -> impl IntoResponse {
    let result: Vec<User> = match state.db.select((format!("alias:{}", user.username_or_email), "FETCH user")).await {
        Ok(users) => users,
        Err(_) => return ( StatusCode::INTERNAL_SERVER_ERROR,Json(ServerError{error:"Erreur lors de la connexion".to_string()}) ).into_response()
    };

    if result[0].password == user.password {
        return (StatusCode::OK, Json(ServerSuccess{message:"Connecté avec succès".to_string()})).into_response()
    }

    (StatusCode::FORBIDDEN, Json(ServerError{error:"Identifiants invalides".to_string()})).into_response()
}