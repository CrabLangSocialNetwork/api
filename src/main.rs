use api_v2_surrealdb::run;

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    run().await
}
