use surrealdb::{Surreal, engine::local::Db};
use tower_cookies::Cookies;

use super::register::User;

pub async fn authentificate(cookies: Cookies, db: Surreal<Db>) -> Result<User, ()> {
    let token = match cookies.get("token") {
        Some(token) => token,
        None => return Err(())
    };

    let mut res = match db.query("SELECT * FROM user WHERE token = $token")
        .bind(("token", token.value()))
        .await {
            Ok(res) => res,
            Err(_) => return Err(())
        };

    let user: Option<User> = match res.take(0) {
        Ok(user) => user,
        Err(_) => return Err(())
    };

    match user {
        Some(user) => Ok(user),
        None => Err(())
    }
}