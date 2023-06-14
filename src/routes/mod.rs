mod get_users;
mod middlewares;
mod register;

use axum::{
    routing::{get, post},
    Router,
};
use surrealdb::{engine::remote::ws::Client, Error, Surreal};

use crate::database::connect;

use register::register;

use self::get_users::get_users;

#[derive(Clone)]
pub struct DbState {
    db: Surreal<Client>,
}

pub async fn create_routes() -> Result<Router, Error> {
    let db = connect().await?;

    db.query("define index userEmailIndex ON TABLE user COLUMNS email UNIQUE")
        .query("define index userUsernameIndex ON TABLE user COLUMNS username UNIQUE")
        .query("define index userTokenIndex ON TABLE user COLUMNS token UNIQUE")
        .await?;

    Ok(Router::new()
        .route("/register", post(register))
        .route("/users", get(get_users))
        .with_state(DbState { db }))
}
