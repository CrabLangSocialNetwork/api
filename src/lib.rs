mod routes;

use std::net::SocketAddr;

use routes::{create_routes, DB};
use surrealdb::{engine::remote::{ws::{Ws}}, opt::auth::Root};

pub async fn run() -> surrealdb::Result<()> {
    DB.connect::<Ws>("localhost:8000").await?;

    DB.signin(Root {
        username: "root",
        password: "root"
    }).await?;

    DB.use_ns("main").use_db("main").await?;

    let app = create_routes();

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}