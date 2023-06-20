mod auth;
mod posts;
mod users;

use axum::{
    http::Method,
    routing::{get, post, put, delete},
    Router,
};
use surrealdb::{engine::local::Db, Error, Surreal};
use tower::ServiceBuilder;
use tower_cookies::CookieManagerLayer;
use tower_http::{cors::{Any, CorsLayer}, services::ServeDir};

use crate::database::connect;

use self::{auth::{register::register, login::login}, users::{get_users::get_users, edit_user::edit_user}, posts::{create_post::create_post, get_posts::get_posts, get_posts_by_user::get_posts_by_user, edit_post::edit_post, delete_post::delete_post}};

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

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_origin(Any);

    Ok(
        Router::new()
            .route("/register", post(register))
            .route("/users", get(get_users))
            .route("/login", post(login))
            .route("/post", post(create_post))
            .route("/posts", get(get_posts))
            .route("/@:username/posts", get(get_posts_by_user))
            .route("/@:username", put(edit_user))
            .route("/posts/:id", put(edit_post))
            .route("/posts/:id", delete(delete_post))
            .nest_service("/media", ServeDir::new("media"))
            .with_state(DbState { db })
            .layer(
                ServiceBuilder::new()
                    .layer(CookieManagerLayer::new())
                    .layer(cors),
            )
    )
}
