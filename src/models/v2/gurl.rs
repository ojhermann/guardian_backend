use crate::schema::*;

#[derive(Debug, PartialEq, Identifiable, Queryable, QueryableByName, Deserialize, Serialize)]
#[table_name = "gurls"]
pub struct Gurl {
    pub id: i32,
    pub url: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub liked: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GurlRequest {
    pub id: Option<i32>,
    pub url: Option<String>,
    pub liked: Option<bool>,
}
