use axum::{response::IntoResponse, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use tower_cookies::Cookies;
use axum::extract::State;

use super::{DbState, authentificate::authentificate, register::PermissionLevel};

#[derive(Deserialize, Serialize)]
pub struct PublicPost {
    content: String,
    images: Vec<String>,
    author: PostAuthor,
    #[serde(default)] has_permission: bool
}

#[derive(Serialize, Deserialize, Default)]
pub struct PostAuthor {
    username: String,
    permission_level: PermissionLevel
}

pub async fn get_posts(cookies: Cookies, State(state): State<DbState>) -> impl IntoResponse {
    let user = authentificate(cookies, &state.db).await;
    let has_full_permission = if user.permission_level >= PermissionLevel::Moderator { true } else { false };

    let mut posts = match state.db.query("SELECT id, content, images, author.username, author.permission_level FROM post").await {
        Ok(res) => res,
        Err(e) => {
            println!("{e}");
            return (StatusCode::INTERNAL_SERVER_ERROR, "Erreur lors de l'obtention des posts").into_response();
        }
    };

    let posts: Vec<PublicPost> = match posts.take(0) {
        Ok(posts) => posts,
        Err(e) => {
            println!("{e}");
            return (StatusCode::INTERNAL_SERVER_ERROR, "Erreur lors de l'obtention des posts").into_response();
        }
    };

    let posts: Vec<PublicPost> = posts.into_iter().map(|mut post| {
        if has_full_permission {
            post.has_permission = true;
        }
        if post.author.username == user.username {
            post.has_permission = true;
        }
        post
    }).collect();

    Json(posts).into_response()
}