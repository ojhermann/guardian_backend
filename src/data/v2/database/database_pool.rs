use crate::models::v2::database_pool_error::DatabasePoolError;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;

pub type DatabasePool = Pool<ConnectionManager<PgConnection>>;

pub fn get(database_url_key: &str) -> Result<DatabasePool, DatabasePoolError> {
    match get_connection_manager(database_url_key) {
        Ok(manager) => r2d2::Pool::builder().build(manager).map_err(Into::into),
        Err(e) => Err(e),
    }
}

fn get_connection_manager(
    database_url_key: &str,
) -> Result<ConnectionManager<PgConnection>, DatabasePoolError> {
    dotenv::var(database_url_key)
        .map(ConnectionManager::<PgConnection>::new)
        .map_err(Into::into)
}

#[cfg(test)]
mod get_connection_manager_test {
    use crate::data::v2::database::database_pool;

    #[test]
    fn it_can_return_connection_manager() {
        dotenv::dotenv().ok();
        let result = database_pool::get_connection_manager("DATABASE_URL");
        assert!(result.is_ok());
    }

    #[test]
    fn it_can_return_error() {
        let result = database_pool::get_connection_manager("this_is_not_an_environment_variable");
        assert!(result.is_err());
    }
}
