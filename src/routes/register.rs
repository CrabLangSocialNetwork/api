use axum::{response::IntoResponse, Json, extract::State, http::StatusCode};
use rand::distributions::{Alphanumeric, DistString};
use serde::{Serialize, Deserialize};

use crate::{routes::{structs::{User, ServerError, ServerSuccess}}};

use super::DbState;

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterUser {
    username: String,
    password: String,
    email: String,
    is_male: Option<bool>
}

pub async fn register(State(state): State<DbState>, Json(user): Json<RegisterUser>) -> impl IntoResponse {
    let user = User {
        id: None,
        email: user.email,
        username: user.username,
        password: user.password,
        is_male: user.is_male,
        token: Alphanumeric.sample_string(&mut rand::thread_rng(), 256)
    };

    let _: User = match state.db.create("user").content(user).await {
        Ok(user) => user,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(ServerError{error:"Erreur lors de la création du compte".to_string()})).into_response()
    };

    (StatusCode::OK, Json(ServerSuccess{message: "Compté créé avec succès".to_string()})).into_response()
}