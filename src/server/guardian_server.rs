use crate::api::v1::gurl::{delete_gurl, get_gurl, insert_gurl};
use crate::data::database_pool;
use actix_web::{middleware, web, App, HttpServer};

pub struct GuardianServer {
    database_url_key: String,
    ip_address: String,
    port: u16,
    workers: usize,
}

impl GuardianServer {
    pub fn from_builder() -> GuardianServerBuilder {
        GuardianServerBuilder::default()
    }

    pub async fn run(&self) -> std::io::Result<()> {
        std::env::set_var("RUST_LOG", "actix_web=info");
        env_logger::init();

        let database_pool = database_pool::get(&*self.database_url_key);

        HttpServer::new(move || {
            App::new()
                .data(database_pool.clone())
                .wrap(middleware::Logger::default())
                .service(
                    web::resource("/gurl/{url_value_or_id}")
                        .route(web::get().to(get_gurl))
                        .route(web::delete().to(delete_gurl)),
                )
                .service(
                    web::resource("/gurl/{url_value}/{liked}").route(web::post().to(insert_gurl)),
                )
        })
        .bind((&*self.ip_address, self.port))?
        .workers(self.workers)
        .run()
        .await
    }
}

pub struct GuardianServerBuilder {
    database_url_key: String,
    ip_address: String,
    port: u16,
    workers: usize,
}

impl GuardianServerBuilder {
    pub fn default() -> Self {
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
        GuardianServer {
            database_url_key: self.database_url_key,
            ip_address: self.ip_address,
            port: self.port,
            workers: self.workers,
        }
    }
}
