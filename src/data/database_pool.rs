use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;

pub fn get(database_url_key: &str) -> Pool<ConnectionManager<PgConnection>> {
    let database_url: String = std::env::var(database_url_key)
        .expect("Your database_url_key should be set in your .env file.\n\
        Local environment variables take precedence over .env values, so modify your .env file if there is an overlap.");

    let manager = ConnectionManager::<PgConnection>::new(database_url);

    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}
