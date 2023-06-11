mod routes;

use std::{net::SocketAddr, process::exit};

use routes::{create_routes};

pub async fn run() -> surrealdb::Result<()> {
    let app = match create_routes().await {
        Ok(app) => app,
        Err(e) => {
            println!("{e}");
            exit(-1);
        }
    };

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}