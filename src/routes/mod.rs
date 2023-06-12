mod login;
mod register;
mod users;
mod structs;

use std::net::SocketAddr;

use axum::{Router, routing::{get, post}, http::{Request, StatusCode}, middleware::{Next, self}, response::Response, extract::ConnectInfo};

use log::info;
use login::login;
use surrealdb::{Surreal, engine::remote::ws::{Client, Ws}, opt::auth::Root};
use tower::ServiceBuilder;

use {register::register, users::get_users};

#[derive(Clone)]
pub struct DbState {
    db: Surreal<Client>
}

async fn log_middleware<B>(ConnectInfo(addr): ConnectInfo<SocketAddr>, request: Request<B>, next: Next<B>) -> Response {
    info!("{}: {} {}", addr, request.method(), request.uri());

    next.run(request).await
}

async fn auth_middleware<B>(request: Request<B>, next: Next<B>) -> Response {
    let token = match request.headers().get("token") {
        Some(token) => token,
        None => return next.run(request).await
    };

    next.run(request).await
}

pub async fn create_routes() -> Result<Router, String> {
    let db = match Surreal::new::<Ws>("localhost:8000").await {
        Ok(db) => db,
        Err(e) => return Err(format!("Erreur lors de la connexion à la base de données : {e}"))
    };

    let shared_state = DbState { db };

    match shared_state.db.signin(Root {
        username: "root",
        password: "root"
    }).await {
        Ok(_) => {},
        Err(e) => return Err(format!("Erreur lors de la connexion à la base de données : {e}"))
    };

    match shared_state.db.use_ns("main").use_db("main").await {
        Ok(_) => {},
        Err(e) => return Err(format!("Erreur lors du choix de la base de données et de l'espace de noms (namespace) : {e}"))
    };

    Ok(
        Router::new()
            .route("/register", post(register))
            .route("/login", post(login))
            .route("/users", get(get_users))
            .with_state(shared_state)
            .layer(
                ServiceBuilder::new()
                    .layer(middleware::from_fn(log_middleware))
            )
    )
}