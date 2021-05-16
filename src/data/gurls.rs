use crate::schema::*;
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    sql_query, sql_types, PgConnection, QueryResult, RunQueryDsl,
};

#[derive(Debug, PartialEq, Identifiable, Queryable, QueryableByName, Deserialize)]
#[table_name = "gurls"]
pub struct Gurl {
    pub id: i32,
    pub url: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub liked: bool,
}

impl Gurl {
    pub fn get(
        url_value: String,
        pooled_connection: PooledConnection<ConnectionManager<PgConnection>>,
    ) -> QueryResult<Vec<Gurl>> {
        sql_query("SELECT * FROM public.get_gurl($1)")
            .bind::<sql_types::Text, _>(url_value)
            .load(&pooled_connection)
    }
}
