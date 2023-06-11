mod login;
mod register;
mod users;
mod structs;

use axum::{Router, routing::{get, post}};

use login::login;
use surrealdb::{Surreal, engine::remote::ws::{Client, Ws}, opt::auth::Root};
use tower::ServiceBuilder;

use {register::register, users::get_users};

use tower_http::trace::TraceLayer;

#[derive(Clone)]
pub struct DbState {
    db: Surreal<Client>
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
                    .layer(TraceLayer::new_for_http())
            )
    )
}