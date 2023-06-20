use std::path::Path;

use axum::extract;
use axum::{extract::State, response::IntoResponse, http::StatusCode, Json};
use chrono::Utc;
use serde::{Serialize, Deserialize};
use surrealdb::sql::Datetime;
use tokio::fs::remove_file;
use tower_cookies::Cookies;

use super::{DbState, register::PermissionLevel, authentificate::authentificate, create_post::{Post, decode_image_and_save_to_disk}};

#[derive(Serialize, Deserialize)]
pub struct EditPost {
    content: String,
    images_url: Vec<String>,
    new_images: Option<Vec<String>>
}

pub async fn edit_post(extract::Path(id): extract::Path<String>, cookies: Cookies, State(state): State<DbState>, Json(mut sent_post): Json<EditPost>) -> impl IntoResponse {
    let now = Datetime(Utc::now());
    let user = authentificate(cookies, &state.db).await;
    if user.permission_level == PermissionLevel::Guest {
        return (StatusCode::FORBIDDEN, "Vous devez être connecté.e pour pouvoir modifier un post").into_response()
    }

    let mut post: Post = match state.db.select(("post", id.clone())).await {
        Ok(post) => post,
        Err(_) => return (StatusCode::NOT_FOUND, "Post non trouvé").into_response()
    };
    if user.permission_level < PermissionLevel::Moderator && post.author != user.id.unwrap() {
        return (StatusCode::FORBIDDEN, "Vous n'avez pas la permission de modifier ce post").into_response();
    }
    sent_post.content = sent_post.content.trim().to_string();
    if sent_post.content.is_empty() || sent_post.content.len() > 500 {
        return (StatusCode::FORBIDDEN, "La taille maximale d'un post est de 500 caractères.").into_response()
    }
    post.content = sent_post.content;

    let mut images: Vec<String> = vec![];

    for image_url in post.images.into_iter() {
        if sent_post.images_url.contains(&image_url) {
            images.push(image_url);
        } else {
            match remove_file(Path::new(&image_url)).await {
                Ok(_) => {},
                Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Erreur lors de la suppression de l'image").into_response()
            };
        }
    }

    if let Some(images_received) = sent_post.new_images {
        for image in images_received.into_iter() {
            match decode_image_and_save_to_disk(image).await {
                Ok(url) => images.push(url),
                Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e).into_response()
            };
        }
    }
    post.images = images;
    post.updated_at = now;
    
    let _: Post = state.db.update(("post", id)).merge(post).await.unwrap();

    (StatusCode::CREATED, "Post modifié avec succès !").into_response()
}