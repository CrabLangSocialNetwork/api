mod database;
mod routes;

use routes::create_routes;
use std::net::SocketAddr;
use surrealdb::Error;

pub async fn run() -> Result<(), Error> {
    let app = create_routes().await?;

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();

    Ok(())
}
