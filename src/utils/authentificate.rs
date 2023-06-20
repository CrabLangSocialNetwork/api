use surrealdb::{Surreal, engine::local::Db};
use tower_cookies::Cookies;

use super::structs::User;

pub async fn authentificate(cookies: Cookies, db: &Surreal<Db>) -> User {
    let token = match cookies.get("token") {
        Some(token) => token,
        None => return User::default()
    };

    let token = token.value();

    let mut res = match db.query("SELECT * FROM user WHERE token == $value")
        .bind(("value", token))
        .await {
            Ok(res) => res,
            Err(e) => {
                println!("{e}");
                return User::default();
            }
        };

    let user: Option<User> = match res.take(0) {
        Ok(user) => user,
        Err(_) => return User::default()
    };

    match user {
        Some(user) => user,
        None => User::default()
    }
}