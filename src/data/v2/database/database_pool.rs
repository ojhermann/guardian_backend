use crate::models::v2::database_pool_error::DatabasePoolError;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;

pub type DatabasePool = Pool<ConnectionManager<PgConnection>>;

pub fn get(database_url_key: &str) -> Result<DatabasePool, DatabasePoolError> {
    match get_connection_manager(database_url_key) {
        Ok(manager) => match r2d2::Pool::builder().build(manager) {
            Ok(database_pool) => Ok(database_pool),
            Err(e) => Err(DatabasePoolError::DatabasePoolBuilder(e)),
        },
        Err(e) => Err(e),
    }
}

fn get_connection_manager(
    database_url_key: &str,
) -> Result<ConnectionManager<PgConnection>, DatabasePoolError> {
    match get_database_url(database_url_key) {
        Ok(database_url) => Ok(ConnectionManager::<PgConnection>::new(database_url)),
        Err(e) => Err(e),
    }
}

fn get_database_url(database_url_key: &str) -> Result<String, DatabasePoolError> {
    match dotenv::var(database_url_key) {
        Ok(database_url) => Ok(database_url),
        Err(e) => Err(DatabasePoolError::DotEnv(e)),
    }
}

#[cfg(test)]
mod get_database_url_test {
    use crate::data::v2::database::database_pool;

    #[test]
    fn it_can_return_string() {
        dotenv::dotenv().ok();
        let result = database_pool::get_database_url("DATABASE_URL");
        assert!(result.is_ok());
    }

    #[test]
    fn it_can_return_error() {
        let result = database_pool::get_database_url("this_is_not_an_environment_variable");
        assert!(result.is_err());
    }
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
