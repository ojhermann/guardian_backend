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

#[derive(Debug, Deserialize, Serialize)]
pub struct GurlsRequest {
    pub start_id: i32,
    pub end_id: i32,
}

#[cfg(test)]
mod gurl_request_tests {
    use crate::models::v2::gurl::GurlRequest;

    #[test]
    fn it_can_deserialize() {
        let data = r#"{"id": 1, "url": "a_fake_url", "liked": true}"#;
        let result = serde_json::from_str::<GurlRequest>(data);
        assert!(result.is_ok());
        let gurl_request = result.unwrap();
        assert_eq!(gurl_request.id, Some(1));
        assert_eq!(gurl_request.url, Some("a_fake_url".to_string()));
        assert_eq!(gurl_request.liked, Some(true));
    }

    #[test]
    fn it_can_deserialize_a_delete_request() {
        let data = r#"{"id": 1}"#;
        let result = serde_json::from_str::<GurlRequest>(data);
        assert!(result.is_ok());
        let gurl_request = result.unwrap();
        assert_eq!(gurl_request.id, Some(1));
        assert!(gurl_request.url.is_none());
        assert!(gurl_request.liked.is_none());
    }

    #[test]
    fn it_can_deserialize_a_get_request() {
        let data = r#"{"url": "a_fake_url"}"#;
        let result = serde_json::from_str::<GurlRequest>(data);
        assert!(result.is_ok());
        let gurl_request = result.unwrap();
        assert!(gurl_request.id.is_none());
        assert_eq!(gurl_request.url.unwrap(), "a_fake_url");
        assert!(gurl_request.liked.is_none());
    }

    #[test]
    fn it_can_deserialize_an_insert_request() {
        let data = r#"{"url": "a_fake_url", "liked": true}"#;
        let result = serde_json::from_str::<GurlRequest>(data);
        assert!(result.is_ok());
        let gurl_request = result.unwrap();
        assert!(gurl_request.id.is_none());
        assert_eq!(gurl_request.url.unwrap(), "a_fake_url");
        assert_eq!(gurl_request.liked.unwrap(), true);
    }
}
