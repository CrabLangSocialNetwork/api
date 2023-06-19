use axum::{response::IntoResponse, extract::{Path, State}, Json};
use tower_cookies::Cookies;

use super::{DbState, get_posts::PublicPost, authentificate::authentificate, register::PermissionLevel};

pub async fn get_posts_by_user(cookies: Cookies, Path(username): Path<String>, State(state): State<DbState>) -> impl IntoResponse {
    let user = authentificate(cookies, &state.db).await;
    let mut res = state.db.query("SELECT id, content, images, author.username, author.permission_level, created_at, updated_at FROM post WHERE author.username == $value")
        .bind(("value", username)).await.unwrap();
    let posts: Vec<PublicPost> = res.take(0).unwrap();

    let posts: Vec<PublicPost> = posts.into_iter().map(|mut post| {
        if user.permission_level >= PermissionLevel::Moderator || post.author.username == user.username {
            post.has_permission = true;
        }
        post
    }).collect();

    Json(posts).into_response()
}