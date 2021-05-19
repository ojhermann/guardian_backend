#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;

pub mod api {
    pub mod v1 {
        pub mod gurl;
    }
}

pub mod data {
    pub mod database_pool;
    pub mod gurl;
    pub mod gurl_error;
}

pub mod schema;

pub mod server {
    pub mod guardian_server;
}