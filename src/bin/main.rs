use guardian_backend::server::guardian_server::GuardianServer;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let gs = GuardianServer::new()
        .database_url_key("DATABASE_URL")
        .ip_address("127.0.0.1")
        .port(8080)
        .workers(8)
        .build();

    gs.run().await
}
