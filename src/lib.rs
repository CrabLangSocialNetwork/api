mod database;
mod routes;
mod utils;
mod structs;

use routes::create_routes;
use tokio::fs::create_dir_all;
use std::{net::SocketAddr, path::Path};

pub async fn run() -> Result<(), surrealdb::Error> {
    match create_dir_all(Path::new("media").join("images")).await {
        Ok(_) => {},
        Err(_) => panic!("Erreur lors de la création des dossiers media/images, vérifiez les autorisations de ce dossier.")
    };

    let app = create_routes().await?;

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();

    Ok(())
}
