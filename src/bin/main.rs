use actix_web::{App, HttpServer};
use guardian_backend::api::v1::gurl::GurlApi;
use guardian_backend::data::database_pool;
use std::io;

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv::dotenv().ok();

    let database_pool = database_pool::get("DATABASE_URL");

    HttpServer::new(move || {
        App::new()
            .data(database_pool.clone())
            .configure(GurlApi::configure)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
