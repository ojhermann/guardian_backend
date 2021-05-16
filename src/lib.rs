#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;

pub mod data {
    pub mod database_pool;
    pub mod gurls;
}

pub mod schema;
