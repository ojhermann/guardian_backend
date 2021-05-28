use guardian_backend::server::v2::guardian_server::GuardianServer;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let gs = GuardianServer::from_builder()
        .database_url_key("DATABASE_URL")
        .ip_address("127.0.0.1")
        .port(8080)
        .workers(8)
        .build();

    gs.run().await
}
