use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use chrono::Utc;
use email_address::EmailAddress;
use hashes::sha3::sha512::hash;
use rand::distributions::{Alphanumeric, DistString};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Datetime;
use tower_cookies::{Cookie, Cookies};

use crate::{routes::DbState, structs::user::{User, PermissionLevel}};

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterUser {
    email: String,
    username: String,
    password: String,
    is_male: Option<bool>,
}

pub fn check_username(username: &str) -> Result<(), String> {
    if username.len() < 5 {
        return Err("Le pseudo doit contenir au moins 5 caractères.".to_string());
    }

    for (i, c) in username.char_indices() {
        if i == 0 {
            if !c.is_alphabetic() {
                return Err("Le pseudo doit commencer par une lettre.".to_string());
            }
        }
        if !c.is_alphanumeric() && c != '_' {
            return Err(
                "Le pseudo ne peut contenir que des lettres, des nombres et des underscores."
                    .to_string(),
            );
        }
    }
    Ok(())
}

pub fn are_credentials_valid(username: &str, password: &str, email: &str) -> Result<(), String> {
    if password.len() < 8 {
        return Err("Le mot de passe doit contenir au moins 8 caractères.".to_string());
    }

    check_username(username)?;

    if !EmailAddress::is_valid(email) {
        return Err("L'email n'est pas valide.".to_string());
    }

    Ok(())
}

pub async fn register(
    cookies: Cookies,
    State(state): State<DbState>,
    Json(register_user): Json<RegisterUser>,
) -> impl IntoResponse {
    let username = register_user.username.to_lowercase();
    match are_credentials_valid(
        &username,
        &register_user.password,
        &register_user.email,
    ) {
        Err(e) => return (StatusCode::FORBIDDEN, e).into_response(),
        Ok(_) => {}
    }

    let hashed_password = hash(register_user.password.as_bytes())
        .into_bytes()
        .to_vec();

    let mut token: String;

    loop {
        token = Alphanumeric.sample_string(&mut rand::thread_rng(), 256);

        let now = Datetime(Utc::now());

        let _: User = match state
            .db
            .create("user")
            .content(User {
                id: None,
                email: register_user.email.clone(),
                username: register_user.username.clone(),
                password: hashed_password.clone(),
                is_male: register_user.is_male,
                token: token.clone(),
                permission_level: PermissionLevel::User,
                created_at: now.clone(),
                updated_at: now.clone()
            })
            .await
        {
            Ok(user) => user,
            Err(e) => {
                let e = e.to_string();
                if e.contains("index `userEmailIndex`") {
                    return (StatusCode::FORBIDDEN, "Adresse email déjà utilisée").into_response();
                }
                if e.contains("index `userUsernameIndex`") {
                    return (StatusCode::FORBIDDEN, "Nom d'utilisateur déjà utilisé")
                        .into_response();
                }
                if e.contains("index `userTokenIndex`") {
                    println!("Le token de connexion généré pour {} existe déjà, création d'un nouveau...", register_user.username);
                    continue;
                }
                println!("Erreur : {e}");
                break;
            }
        };
        break;
    }
    (
        StatusCode::CREATED,
        cookies.add(Cookie::new("token", token)),
    )
        .into_response()
}
