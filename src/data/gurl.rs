use crate::schema::*;
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    sql_query, sql_types, PgConnection, QueryResult, RunQueryDsl,
};

pub type PooledConn = PooledConnection<ConnectionManager<PgConnection>>;

#[derive(Debug, PartialEq, Identifiable, Queryable, QueryableByName, Deserialize, Serialize)]
#[table_name = "gurls"]
pub struct Gurl {
    pub id: i32,
    pub url: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub liked: bool,
}

impl Gurl {
    pub fn delete(id_value: i32, pooled_connection: &PooledConn) -> QueryResult<usize> {
        sql_query("SELECT public.delete_gurl($1)")
            .bind::<sql_types::Integer, _>(id_value)
            .execute(pooled_connection)
    }

    pub fn get(url_value: String, pooled_connection: &PooledConn) -> QueryResult<Vec<Gurl>> {
        sql_query("SELECT * FROM public.get_gurl($1)")
            .bind::<sql_types::Text, _>(url_value)
            .load(pooled_connection)
    }

    pub fn insert(
        url_value: String,
        liked: bool,
        pooled_connection: &PooledConn,
    ) -> QueryResult<usize> {
        sql_query("SELECT public.insert_gurl($1, $2)")
            .bind::<sql_types::Text, _>(url_value)
            .bind::<sql_types::Bool, _>(liked)
            .execute(pooled_connection)
    }
}
