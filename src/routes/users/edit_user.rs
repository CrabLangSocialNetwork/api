use axum::{response::IntoResponse, extract::{Path, State}, Json, http::StatusCode};
use chrono::Utc;
use email_address::EmailAddress;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Datetime;
use hashes::sha3::sha512::hash;
use tower_cookies::Cookies;

use crate::{routes::{auth::register::check_username, DbState}, utils::authentificate::authentificate, structs::user::{User, PermissionLevel}};

#[derive(Serialize, Deserialize)]
pub struct EditUser {
    email: Option<String>,
    pub(crate) username: Option<String>,
    pub(crate) password: Option<String>
}

pub async fn edit_user(cookies: Cookies, Path(username): Path<String>, State(state): State<DbState>, Json(edit_user): Json<EditUser>) -> impl IntoResponse {
    let now = Datetime(Utc::now());
    let username = username.to_lowercase();
    let request_user = authentificate(cookies, &state.db).await;
    let mut res = match state.db.query("SELECT * FROM user WHERE username == $value").bind(("value", username)).await {
        Ok(res) => res,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Erreur lors de l'obtention du compte").into_response()
    };

    let mut user: User = match match res.take(0) {
        Ok(user) => user,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Erreur lors de l'obtention du compte").into_response()
    } {
        Some(user) => user,
        None => return StatusCode::NOT_FOUND.into_response()
    };

    if user.username != request_user.username && request_user.permission_level < PermissionLevel::Moderator {
        return (StatusCode::FORBIDDEN, "Vous n'avez pas l'autorisation de modifier cet utilisateur, veuillez vérifier que vous êtes bien connecté").into_response()
    }

    let mut modified = false;

    if let Some(email) = edit_user.email {
        if user.email != email {
            if EmailAddress::is_valid(&email) {
                user.email = email;
                modified = true;
            }
        }
    }
    
    if let Some(username) = edit_user.username {
        if user.username != username {
            match check_username(&username) {
                Ok(_) => {
                    user.username = username;
                    modified = true;
                },
                Err(_) => {}
            }
        }
    }

    if let Some(password) = edit_user.password {
        let password = hash(password.as_bytes())
            .into_bytes()
            .to_vec();
        if user.password != password {
            if password.len() >= 8 {
                user.password = password;
                modified = true;
            }
        }   
    }

    if modified {
        user.updated_at = now;

        let _: User = match state.db.update(("user", user.id.clone().unwrap().id)).merge(user).await {
            Ok(user) => user,
            Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Erreur lors de la modification de l'utilisateur").into_response()
        };

        return "Utilisateur modifié avec succès".into_response()
    }

    StatusCode::NOT_MODIFIED.into_response()
}