use crate::data::v2::database::database_pool::DatabasePool;
use crate::models::v2::database_method_error::DatabaseMethodError;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::PgConnection;

pub fn get(
    database_pool: actix_web::web::Data<DatabasePool>,
) -> Result<PooledConnection<ConnectionManager<PgConnection>>, DatabaseMethodError> {
    database_pool.get().map_err(Into::into)
}
