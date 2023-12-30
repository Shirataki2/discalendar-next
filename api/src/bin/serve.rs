#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    api::run().await
}
