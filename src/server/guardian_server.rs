use crate::api;
use crate::data::database_pool;
use actix_web::{middleware, App, HttpServer};

pub struct GuardianServer {
    database_url_key: String,
    ip_address: String,
    port: u16,
    workers: usize,
    logging_key: String,
    logging_value: String,
}

impl GuardianServer {
    pub fn from_builder() -> GuardianServerBuilder {
        GuardianServerBuilder::default()
    }

    pub async fn run(&self) -> std::io::Result<()> {
        std::env::set_var(&*self.logging_key, &*self.logging_value);
        env_logger::init();

        let database_pool = database_pool::get(&*self.database_url_key);

        HttpServer::new(move || {
            App::new()
                .data(database_pool.clone())
                .wrap(middleware::Logger::default())
                .configure(api::v1::gurl::configure)
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
    logging_key: String,
    logging_value: String,
}

impl GuardianServerBuilder {
    pub fn default() -> Self {
        Self {
            database_url_key: String::from("DATABASE_URL"),
            ip_address: String::from("127.0.0.1"),
            port: 8080,
            workers: 8,
            logging_key: String::from("RUST_LOG"),
            logging_value: String::from("actix_web=info"),
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

    pub fn logging_key(mut self, logging_key: String) -> Self {
        self.logging_key = logging_key;
        self
    }

    pub fn logging_value(mut self, logging_value: String) -> Self {
        self.logging_value = logging_value;
        self
    }

    pub fn build(self) -> GuardianServer {
        GuardianServer {
            database_url_key: self.database_url_key,
            ip_address: self.ip_address,
            port: self.port,
            workers: self.workers,
            logging_key: self.logging_key,
            logging_value: self.logging_value,
        }
    }
}
