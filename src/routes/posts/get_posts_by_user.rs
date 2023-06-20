use axum::{response::IntoResponse, extract::{Path, State}, Json, http::StatusCode};
use tower_cookies::Cookies;

use crate::routes::{auth::{authentificate::authentificate, register::PermissionLevel}, DbState};

use super::get_posts::PublicPost;

pub async fn get_posts_by_user(cookies: Cookies, Path(username): Path<String>, State(state): State<DbState>) -> impl IntoResponse {
    let username = username.to_lowercase();
    let user = authentificate(cookies, &state.db).await;
    let mut res = match state.db.query("SELECT meta::id(id) AS id, content, images, author.username, author.permission_level, created_at, updated_at FROM post WHERE author.username == $value")
        .bind(("value", username)).await {
            Ok(res) => res,
            Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Erreur lors de l'obtention des posts").into_response()
        };
    let posts: Vec<PublicPost> = match res.take(0) {
        Ok(posts) => posts,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Erreur lors de l'obtention des posts").into_response()
    };

    let posts: Vec<PublicPost> = posts.into_iter().map(|mut post| {
        if user.permission_level >= PermissionLevel::Moderator || post.author.username == user.username {
            post.has_permission = true;
        }
        post
    }).collect();

    Json(posts).into_response()
}