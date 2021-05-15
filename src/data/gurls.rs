use crate::schema::*;

#[derive(Debug, PartialEq, Identifiable, Queryable, Deserialize)]
#[table_name = "gurls"]
pub struct Gurl {
    id: i32,
    url: String,
    created_at: chrono::DateTime<chrono::Utc>,
    liked: bool,
}

impl Gurl {
    fn new(id: i32, url: String, created_at: chrono::DateTime<chrono::Utc>, liked: bool) -> Self {
        Self {
            id,
            url,
            created_at,
            liked,
        }
    }

    // methods below interact with the database

    // like, dislike, like_many, dislike_many

    // get

    // delete, delete_many
}
