use actix_web::{App, HttpServer};
use guardian_backend::api::v1::gurl::GurlApi;
use guardian_backend::data::database_pool;

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

pub struct GuardianServer {
    database_url_key: String,
    ip_address: String,
    port: u16,
    workers: usize,
}

pub struct GuardianServerBuilder {
    database_url_key: String,
    ip_address: String,
    port: u16,
    workers: usize,
}

impl GuardianServerBuilder {
    pub fn new() -> Self {
        Self {
            database_url_key: String::from("DATABASE_URL"),
            ip_address: String::from("127.0.0.1"),
            port: 8080,
            workers: 8,
        }
    }

    pub fn database_url_key(mut self, database_url_key: &str) -> Self {
        self.database_url_key = String::from(database_url_key);
        self
    }

    pub fn ip_address(mut self, ip_address: &str) -> Self {
        self.ip_address = String::from(ip_address);
        self
    }

    pub fn port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    pub fn workers(mut self, workers: usize) -> Self {
        self.workers = workers;
        self
    }

    pub fn build(self) -> GuardianServer {
        GuardianServer{
            database_url_key: self.database_url_key,
            ip_address: self.ip_address,
            port: self.port,
            workers: self.workers,
        }
    }
}

impl GuardianServer {
    pub fn new() -> GuardianServerBuilder {
        GuardianServerBuilder::new()
    }

    pub async fn run(&self) -> std::io::Result<()> {
        let database_pool = database_pool::get(&*self.database_url_key);

        HttpServer::new(move || {
            App::new()
                .data(database_pool.clone())
                .configure(GurlApi::configure)
        })
        .bind((&*self.ip_address, self.port))?
        .workers(   self.workers)
        .run()
        .await
    }
}
