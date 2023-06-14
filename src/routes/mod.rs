mod get_users;
mod login;
mod register;

use axum::{
    routing::{get, post},
    Router,
};
use surrealdb::{Error, Surreal, engine::local::Db};
use tower_cookies::CookieManagerLayer;

use crate::database::connect;

use register::register;

use self::{get_users::get_users, login::login};

#[derive(Clone)]
pub struct DbState {
    db: Surreal<Db>,
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
        .route("/login", post(login))
        .with_state(DbState { db })
        .layer(CookieManagerLayer::new()))
}
