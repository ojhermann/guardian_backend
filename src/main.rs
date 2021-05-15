#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;

mod data;
mod schema;

use crate::data::database_pool;

fn main() {
    dotenv::dotenv().ok();

    let database_pool = crate::database_pool::get("DATABASE_URL");

    println!(
        "Number of connnections: {}",
        database_pool.state().connections
    );
    println!(
        "Number of idle connections: {}",
        database_pool.state().idle_connections
    );
}
