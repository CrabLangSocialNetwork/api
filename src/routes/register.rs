use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use email_address::EmailAddress;
use hashes::sha3::sha512::hash;
use rand::distributions::{Alphanumeric, DistString};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
use tower_cookies::{Cookie, Cookies};

use super::DbState;

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterUser {
    email: String,
    username: String,
    password: String,
    is_male: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    id: Option<Thing>,
    email: String,
    username: String,
    pub(crate) password: Vec<u8>,
    is_male: Option<bool>,
    pub(crate) token: String,
}

fn are_credentials_valid(username: &str, password: &str, email: &str) -> Result<(), String> {
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

    if password.len() < 8 {
        return Err("Le mot de passe doit contenir au moins 8 caractères.".to_string());
    }

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
    match are_credentials_valid(
        &register_user.username,
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
                println!("Token already exists, creating another...");
                continue;
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
