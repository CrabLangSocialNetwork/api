mod routes;

use std::{net::SocketAddr, process::exit};

use log::LevelFilter;
use routes::create_routes;

pub async fn run() -> surrealdb::Result<()> {
    match simple_logging::log_to_file(format!(".log"), LevelFilter::Info) {
        Ok(_) => {},
        Err(e) => {
            println!("{e}");
            exit(-1);
        }
    };

    let app = match create_routes().await {
        Ok(app) => app,
        Err(e) => {
            println!("{e}");
            exit(-1);
        }
    };

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();

    Ok(())
}