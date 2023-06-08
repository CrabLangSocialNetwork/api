use std::{net::SocketAddr};

use axum::{
    Router,
    routing::{post, get}, extract::{self, State}, response::IntoResponse
};
use serde::{Serialize, Deserialize};
use surrealdb::{Surreal, engine::remote::ws::Client, opt::auth::Root, sql::Thing};

pub static DB: Surreal<Client> = Surreal::init();

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: Option<Thing>,
    username: String,
    password: String,
    email: String,
    is_male: Option<bool>
}

#[derive(Debug, Serialize, Deserialize)]
struct PublicUser {
    id: Option<Thing>,
    username: String,
    is_male: Option<bool>
}

#[derive(Debug, Serialize, Deserialize)]
struct LoginUser {
    username: String,
    password: String
}

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    DB.connect::<surrealdb::engine::local::File>("./test.db").await?;

    DB.use_ns("main").use_db("main").await?;

    let app = Router::new()
        // .route("/register", post(register))
        .route("/users", get(get_users));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn register(State(state): State<Surreal<Client>>, extract::Json(user): extract::Json<User>) -> impl IntoResponse {
    println!("{user:?}");

    let result: User = match state.create("user").content(user).await {
        Ok(user) => user,
        Err(e) => return e.to_string()
    };

    println!("{result:?}");

    "salut".to_string()
}

async fn login(extract::Json(user): extract::Json<User>) -> impl IntoResponse {
    println!("{user:?}");

    let result: User = match DB.create("user").content(user).await {
        Ok(user) => user,
        Err(e) => return e.to_string()
    };

    println!("{result:?}");

    "salut".to_string()
}

async fn get_users() -> impl IntoResponse {
    let result: Vec<PublicUser> = match DB.select("user").await {
        Ok(users) => users,
        Err(e) => return e.to_string()
    };

    println!("{result:?}");

    match serde_json::to_string(&result) {
        Ok(users) => users,
        Err(e) => e.to_string()
    }
}
