use axum::{extract::{Path, State}, response::IntoResponse, http::StatusCode};
use serde::Deserialize;
use tower_cookies::Cookies;

use crate::{routes::DbState, utils::{authentificate::authentificate, structs::{PermissionLevel, Post}}};

#[derive(Debug, Deserialize)]
pub struct DeletePostUser {
    author_username: String
}

pub async fn delete_post(cookies: Cookies, Path(id): Path<String>, State(state): State<DbState>) -> impl IntoResponse {
    let user = authentificate(cookies, &state.db).await;
    let mut res = match state.db.query(format!("SELECT author.username AS author_username FROM post:{id}")).await {
        Ok(res) => res,
        Err(e) => {
            println!("{e}");
            return (StatusCode::INTERNAL_SERVER_ERROR, "Erreur lors de l'obtention du post").into_response();
        }
    };
    let post: DeletePostUser = match match res.take(0) {
        Ok(post) => post,
        Err(e) => {
            println!("{e}");
            return (StatusCode::INTERNAL_SERVER_ERROR, "Erreur lors de l'obtention du post").into_response();
        }
    } {
        Some(post) => post,
        None => return (StatusCode::NOT_FOUND, "Post introuvable").into_response()
    };

    if user.username != post.author_username && user.permission_level < PermissionLevel::Moderator {
        return (StatusCode::FORBIDDEN, "Vous n'avez pas la permission de supprimer ce post").into_response()
    }

    let _: Post = match state.db.delete(("post", id)).await {
        Ok(post) => post,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Erreur lors de la suppression du post").into_response()
    };

    "Post supprimé avec succès".into_response()
}