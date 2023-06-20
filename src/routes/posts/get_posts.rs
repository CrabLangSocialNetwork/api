use axum::{response::IntoResponse, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Datetime;
use tower_cookies::Cookies;
use axum::extract::State;

use crate::routes::{auth::{register::PermissionLevel, authentificate::authentificate}, DbState};

#[derive(Deserialize, Serialize)]
pub struct PublicPost {
    id: String,
    content: String,
    //vector of images links
    images: Vec<String>,
    pub(crate) author: PostAuthor,
    #[serde(default)] pub(crate) has_permission: bool,
    created_at: Datetime,
    updated_at: Datetime
}

#[derive(Serialize, Deserialize, Default)]
pub struct PostAuthor {
    pub(crate) username: String,
    permission_level: PermissionLevel
}

pub async fn get_posts(cookies: Cookies, State(state): State<DbState>) -> impl IntoResponse {
    let user = authentificate(cookies, &state.db).await;
    let has_full_permission = if user.permission_level >= PermissionLevel::Moderator { true } else { false };

    let mut posts = match state.db.query("SELECT meta::id(id) AS id, content, images, author.username, author.permission_level, created_at, updated_at FROM post").await {
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
        if has_full_permission || post.author.username == user.username {
            post.has_permission = true;
        }
        post
    }).collect();

    Json(posts).into_response()
}