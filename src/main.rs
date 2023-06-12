use api::run;

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    run().await
}
