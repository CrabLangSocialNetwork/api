mod get_users;
mod login;
mod register;

use axum::{
    routing::{get, post},
    Router,
};
use surrealdb::{engine::remote::ws::Client, Error, Surreal};

use crate::database::connect;

use login::login;
use register::register;

use self::get_users::get_users;

#[derive(Clone)]
pub struct DbState {
    db: Surreal<Client>,
}

pub async fn create_routes() -> Result<Router, Error> {
    let db = connect().await?;

    Ok(Router::new()
        .route("/login", post(login))
        .route("/register", post(register))
        .route("/users", get(get_users))
        .with_state(DbState { db }))
}
