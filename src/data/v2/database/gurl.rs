use crate::models::v2::gurl::Gurl;
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    sql_query, sql_types, PgConnection, QueryResult, RunQueryDsl,
};

pub fn delete(
    id_value: i32,
    pooled_connection: &PooledConnection<ConnectionManager<PgConnection>>,
) -> QueryResult<usize> {
    sql_query("SELECT public.delete_gurl($1)")
        .bind::<sql_types::Integer, _>(id_value)
        .execute(pooled_connection)
}

pub fn get(
    url_value: String,
    pooled_connection: &PooledConnection<ConnectionManager<PgConnection>>,
) -> QueryResult<Vec<Gurl>> {
    sql_query("SELECT * FROM public.get_gurl($1)")
        .bind::<sql_types::Text, _>(url_value)
        .load(pooled_connection)
}

pub fn get_most_recently_added_gurl(
    pooled_connection: &PooledConnection<ConnectionManager<PgConnection>>,
) -> QueryResult<Vec<Gurl>> {
    // sql_query return type can only be a struct which implements QueryableByName
    // https://docs.rs/diesel/1.4.6/diesel/query_dsl/trait.RunQueryDsl.html#method.load
    // https://docs.rs/diesel/1.4.6/diesel/fn.sql_query.html
    sql_query("SELECT * FROM public.get_most_recently_added_gurl()").load(pooled_connection)
}

pub fn insert(
    url_value: String,
    liked: bool,
    pooled_connection: &PooledConnection<ConnectionManager<PgConnection>>,
) -> QueryResult<usize> {
    sql_query("SELECT public.insert_gurl($1, $2)")
        .bind::<sql_types::Text, _>(url_value)
        .bind::<sql_types::Bool, _>(liked)
        .execute(pooled_connection)
}
