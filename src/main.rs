use serde::{Serialize, Deserialize};
use surrealdb::{Surreal, engine::remote::ws::Ws, opt::auth::Root, sql::Thing};

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: Option<Thing>,
    username: String,
    password: String,
    email: String
}

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    let db = Surreal::new::<Ws>("localhost:8000").await?;
    
    db.signin(Root {
        username: "root",
        password: "root"
    }).await?;

    db.use_ns("test").use_db("test").await?;

    let created: User = db.create("user")
        .content(User {
            id: None,
            username: "username".to_string(),
            password: "password".to_string(),
            email: "test@example.com".to_string()
        })
        .await?;
    dbg!(created);

    Ok(())
}
