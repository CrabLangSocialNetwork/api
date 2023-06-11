use axum::{response::IntoResponse, Json};
use rand::distributions::{Alphanumeric, DistString};
use serde::{Serialize, Deserialize};

use crate::routes::{DB, structs::{User, ServerError, ServerSuccess}};

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterUser {
    username: String,
    password: String,
    email: String,
    is_male: Option<bool>
}

pub async fn register(Json(user): Json<RegisterUser>) -> impl IntoResponse {
    let user = User {
        id: None,
        email: user.email,
        username: user.username,
        password: user.password,
        is_male: user.is_male,
        token: Alphanumeric.sample_string(&mut rand::thread_rng(), 256)
    };

    let _: User = match DB.create("user").content(user).await {
        Ok(user) => user,
        Err(_) => return Json(ServerError{error:"Erreur lors de la création du compte".to_string()}).into_response()
    };

    Json(ServerSuccess{message: "Compté créé avec succès".to_string()}).into_response()
}