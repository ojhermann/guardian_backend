extern crate guardian_backend;

use guardian_backend::data::v2::database::database_pool;

#[test]
pub fn it_can_connect() {
    dotenv::dotenv().ok();

    let result = database_pool::get("DATABASE_URL");
    assert!(result.is_ok());

    let database_pool = result.ok().unwrap();
    assert_eq!(database_pool.state().connections, 10);
    assert_eq!(database_pool.state().idle_connections, 10);
}
