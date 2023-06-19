use axum::{extract::{State, Path}, response::IntoResponse, http::StatusCode, Json};
use chrono::Utc;
use serde::{Serialize, Deserialize};
use surrealdb::sql::Datetime;
use tower_cookies::Cookies;

use super::{DbState, register::PermissionLevel, authentificate::authentificate, create_post::Post};

#[derive(Serialize, Deserialize)]
pub struct EditPost {
    content: String
}

pub async fn edit_post(Path(id): Path<String>, cookies: Cookies, State(state): State<DbState>, Json(mut sent_post): Json<EditPost>) -> impl IntoResponse {
    let now = Datetime(Utc::now());
    let user = authentificate(cookies, &state.db).await;
    if user.permission_level == PermissionLevel::Guest {
        return (StatusCode::FORBIDDEN, "Vous devez être connecté.e pour pouvoir modifier un post").into_response()
    }

    let mut post: Post = state.db.select(("post", id.clone())).await.unwrap();
    if user.permission_level < PermissionLevel::Moderator && post.author != user.id.unwrap() {
        return (StatusCode::FORBIDDEN, "Vous n'avez pas la permission de modifier ce post").into_response();
    }
    sent_post.content = sent_post.content.trim().to_string();
    if sent_post.content.is_empty() || sent_post.content.len() > 500 {
        return (StatusCode::FORBIDDEN, "La taille maximale d'un post est de 500 caractères.").into_response()
    }
    post.content = sent_post.content;
    post.updated_at = now;
    
    let _: Post = state.db.update(("post", id)).merge(post).await.unwrap();

    (StatusCode::CREATED, "Post modifié avec succès !").into_response()
}