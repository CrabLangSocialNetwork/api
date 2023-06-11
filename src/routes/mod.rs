mod login;
mod register;
mod users;
mod structs;

use axum::{Router, routing::{get, post}};

use login::login;
use surrealdb::{Surreal, engine::remote::ws::Client};

use {register::register, users::get_users};

pub static DB: Surreal<Client> = Surreal::init();

pub fn create_routes() -> Router {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/users", get(get_users))
}