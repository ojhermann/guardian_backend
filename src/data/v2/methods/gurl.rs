use crate::data::v2::database;
use crate::models::v2::{database_method_error::DatabaseMethodError, gurl::Gurl};
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};

pub fn delete(
    id_value: i32,
    pooled_connection: &PooledConnection<ConnectionManager<PgConnection>>,
) -> Result<usize, DatabaseMethodError> {
    database::gurl::delete(id_value, pooled_connection).map_err(Into::into)
}

pub fn get(
    url_value: String,
    pooled_connection: &PooledConnection<ConnectionManager<PgConnection>>,
) -> Result<Vec<Gurl>, DatabaseMethodError> {
    database::gurl::get(url_value, pooled_connection).map_err(Into::into)
}

pub fn get_gurls(
    start_id: i32,
    end_id: i32,
    pooled_connection: &PooledConnection<ConnectionManager<PgConnection>>,
) -> Result<Vec<Gurl>, DatabaseMethodError> {
    database::gurl::get_gurls(start_id, end_id, pooled_connection).map_err(Into::into)
}

pub fn get_most_recently_added_gurl(
    pooled_connection: &PooledConnection<ConnectionManager<PgConnection>>,
) -> Result<Option<Gurl>, DatabaseMethodError> {
    database::gurl::get_most_recently_added_gurl(pooled_connection)
        .map_err(Into::into)
        .map(|mut gurl_vector| gurl_vector.pop())
}

pub fn insert(
    url_value: String,
    liked: bool,
    pooled_connection: &PooledConnection<ConnectionManager<PgConnection>>,
) -> Result<usize, DatabaseMethodError> {
    database::gurl::insert(url_value, liked, pooled_connection).map_err(Into::into)
}
