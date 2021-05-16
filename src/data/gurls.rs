use crate::schema::*;

#[derive(Debug, PartialEq, Identifiable, Queryable, Deserialize)]
#[table_name = "gurls"]
pub struct Gurl<'a> {
    pub id: i32,
    pub url: &'a str,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub liked: bool,
}
