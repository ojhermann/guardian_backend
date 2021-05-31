use crate::data::v2::database::database_pool::DatabasePool;
use crate::models::v2::database_method_error::DatabaseMethodError;

pub fn run(dp: &DatabasePool) -> Result<(), DatabaseMethodError> {
    match dp.get().map_err(Into::into) {
        Ok(conn) => match diesel_migrations::run_pending_migrations(&conn).map_err(Into::into) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        },
        Err(e) => Err(e),
    }
}
