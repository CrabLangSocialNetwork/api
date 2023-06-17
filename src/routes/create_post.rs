use std::{io::Cursor, path::Path};

use axum::{extract::State, response::IntoResponse, Json, http::StatusCode};
use base64::{engine::general_purpose, Engine};
use rand::distributions::{Alphanumeric, DistString};
use serde::{Serialize, Deserialize};
use image::io::Reader as ImageReader;
use surrealdb::sql::Thing;
use tokio::fs::try_exists;
use tower_cookies::Cookies;

use super::{DbState, is_token_valid::authentificate};

#[derive(Serialize, Deserialize)]
pub struct CreatePost {
    content: String,
    images: Option<Vec<String>>
}

#[derive(Serialize, Deserialize)]
pub struct Post {
    content: String,
    images: Vec<String>,
    author: Thing
}

async fn decode_image(encoded_image: String) -> Result<String, String> {
    let image = match general_purpose::STANDARD.decode(encoded_image) {
        Ok(image) => image,
        Err(_) => return Err("Erreur lors du décodage de l'image".to_string())
    };

    let image_reader = match ImageReader::new(Cursor::new(image)).with_guessed_format() {
        Ok(image_reader) => image_reader,
        Err(_) => return Err("Erreur lors de la création du lecteur d'image".to_string())
    };

    let image_format = match image_reader.format() {
        Some(format) => format,
        None => return Err("Erreur lors de la détection du format de l'image".to_string())
    };
    
    let img = match image_reader.decode() {
        Ok(image) => image,
        Err(_) => return Err("Erreur lors du décodage de l'image".to_string())
    };

    loop {
        let path = Path::new("media").join("images").join(format!("{}.{}",Alphanumeric.sample_string(&mut rand::thread_rng(), 38), image_format.extensions_str()[0]));

        match try_exists(&path).await {
            Ok(exists) => if exists {continue},
            Err(_) => {}
        }

        match img.save_with_format(&path, image_format) {
            Ok(_) => {},
            Err(e) => {
                println!("{e}");
                return Err("Erreur lors de l'enregistrement de l'image".to_string())
            }
        };

        return Ok(path.to_str().unwrap().to_string());
    }
}

pub async fn create_post(cookies: Cookies, State(state): State<DbState>, Json(post): Json<CreatePost>) -> impl IntoResponse {
    let author = match authentificate(cookies, state.db.clone()).await {
        Ok(user) => user,
        Err(_) => return (StatusCode::FORBIDDEN, "Vous devez être connecté.e pour pouvoir poster un post").into_response()
    };
    
    if post.content.len() > 500 {
        return (StatusCode::FORBIDDEN, "Le post ne peut pas dépasser 500 caractères.").into_response();
    }

    let mut images_url: Vec<String> = vec![];

    if let Some(images) = post.images {
        for encoded_image in images.into_iter() {
            let image_url = match decode_image(encoded_image).await {
                Ok(decoded_image) => decoded_image,
                Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e).into_response()
            };
            images_url.push(image_url);
        }
    }

    let author = match author.id {
        Some(id) => id,
        None => return (StatusCode::INTERNAL_SERVER_ERROR, "Erreur de connexion, vérifiez que vous êtes bien connecté.e").into_response()
    };

    let _: CreatePost = match state.db.create("post").content(Post {
        content: post.content,
        images: images_url,
        author
    }).await {
        Ok(post) => post,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Erreur lors de la publication du post").into_response()
    };

    (StatusCode::CREATED, "Post publié avec succès").into_response()
}