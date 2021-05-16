extern crate guardian_backend;

use self::guardian_backend::data::gurls::Gurl;
use guardian_backend::data::database_pool;
use guardian_backend::data::gurls;

#[test]
pub fn get_works() {
    dotenv::dotenv().ok();
    let database_url_key = "DATABASE_URL";
    let dp = database_pool::get(database_url_key);

    let pooled_connection = dp.get().unwrap();
    let query_result = gurls::Gurl::get("get_gurl_works".to_string(), pooled_connection);

    assert!(query_result.is_ok());

    let results: Vec<Gurl> = query_result.unwrap();
    assert_eq!(results.len(), 2);

    results
        .iter()
        .for_each(|gurl| assert_eq!(gurl.url, "get_gurl_works"));
}


#[test]
pub fn insert_works() {
    dotenv::dotenv().ok();
    let database_url_key = "DATABASE_URL";
    let dp = database_pool::get(database_url_key);

    let pooled_connection = dp.get().unwrap();
    let query_result = gurls::Gurl::insert("insert_works".to_string(), true, pooled_connection);

    assert!(query_result.is_ok());

    assert_eq!(query_result.unwrap(), 1);
}