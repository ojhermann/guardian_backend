extern crate guardian_backend;

use guardian_backend::data::v1::database_pool;

#[test]
pub fn it_can_connect() {
    dotenv::dotenv().ok();

    let database_url_key = "DATABASE_URL";

    let database_pool = database_pool::get(database_url_key);

    assert_eq!(database_pool.state().connections, 10);
    assert_eq!(database_pool.state().idle_connections, 10);
}

#[test]
#[should_panic(expected = "Your database_url_key should be set in your .env file.\n\
Local environment variables take precedence over .env values, so modify your .env file if there is an overlap.")]
pub fn it_panics_when_given_a_bad_url_key() {
    dotenv::dotenv().ok();

    let bogus_database_url_key = "bogus_database_url_key";

    match std::env::var(bogus_database_url_key) {
        Ok(_) => assert_eq!(
            "The bogus_database_url_key is in .env; remove it.",
            bogus_database_url_key
        ),
        Err(_) => {
            database_pool::get(bogus_database_url_key);
        }
    }
}
