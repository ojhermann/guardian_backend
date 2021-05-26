use crate::schema::*;

#[derive(Debug, PartialEq, Identifiable, Queryable, QueryableByName, Deserialize, Serialize)]
#[table_name = "gurls"]
pub struct Gurl {
    pub id: i32,
    pub url: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub liked: bool,
}
