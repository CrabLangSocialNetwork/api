use surrealdb::{
    engine::local::Db,
    Error, Surreal,
};

pub async fn connect() -> Result<Surreal<Db>, Error> {
    let db = Surreal::new::<surrealdb::engine::local::File>("./database.db").await?;

    db.use_ns("test").use_db("test").await?;

    Ok(db)
}
