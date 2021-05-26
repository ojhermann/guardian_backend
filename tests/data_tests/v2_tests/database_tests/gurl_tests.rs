use guardian_backend::data::v2::database::database_pool::DatabasePool;
use guardian_backend::data::v2::database::{database_pool, gurl};
use guardian_backend::models::v2::gurl::Gurl;

#[test]
pub fn insert_get_and_delete_work() {
    dotenv::dotenv().ok();

    let database_url_key = "DATABASE_URL";
    let url_value = "insert_get_and_delete_work";
    let dp_result = database_pool::get(database_url_key);
    assert!(dp_result.is_ok());
    let database_pool = dp_result.unwrap();

    insert_works(&database_pool, url_value);
    insert_works(&database_pool, url_value);

    get_works(&database_pool, url_value, 2)
        .iter()
        .for_each(|id_value| delete_works(&database_pool, *id_value));

    get_works(&database_pool, url_value, 0);
}

#[test]
pub fn get_most_recently_added_gurl_works() {
    dotenv::dotenv().ok();

    let database_url_key = "DATABASE_URL";
    let url_value = "insert_get_and_delete_work";
    let dp_result = database_pool::get(database_url_key);
    assert!(dp_result.is_ok());
    let database_pool = dp_result.unwrap();

    let pooled_connection = database_pool.get().unwrap();
    let _insert_result = gurl::insert(url_value.to_string(), true, &pooled_connection);

    let get_query_result = gurl::get(url_value.to_string(), &pooled_connection);
    assert!(get_query_result.is_ok());
    let vec_of_gurls = get_query_result.unwrap();
    assert_eq!(vec_of_gurls.len(), 1);
    let id_of_most_recently_added_gurl = vec_of_gurls.get(0).unwrap().id;

    let tested_fnc_query_result = gurl::get_most_recently_added_gurl(&pooled_connection);
    assert!(tested_fnc_query_result.is_ok());
    let vec_of_results = tested_fnc_query_result.unwrap();
    assert_eq!(vec_of_results.len(), 1);
    let id_of_tested_query = vec_of_results.get(0).unwrap().id;
    assert_eq!(id_of_most_recently_added_gurl, id_of_tested_query);

    let _delete_result = gurl::delete(id_of_most_recently_added_gurl, &pooled_connection);
}

fn insert_works(database_pool: &DatabasePool, url_value: &str) {
    let pooled_connection = database_pool.get().unwrap();
    let result = gurl::insert(url_value.to_string(), true, &pooled_connection);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 1);
}

fn get_works(database_pool: &DatabasePool, url_value: &str, expected_size: usize) -> Vec<i32> {
    let pooled_connection = database_pool.get().unwrap();

    let result = gurl::get(url_value.to_string(), &pooled_connection);
    assert!(result.is_ok());

    let gurl_vector: Vec<Gurl> = result.unwrap();
    assert_eq!(gurl_vector.len(), expected_size);

    gurl_vector
        .iter()
        .for_each(|gurl| assert_eq!(gurl.url, url_value));

    gurl_vector.iter().map(|gurl| gurl.id).collect()
}

fn delete_works(database_pool: &DatabasePool, id_value: i32) {
    let pooled_connection = database_pool.get().unwrap();
    let result = gurl::delete(id_value, &pooled_connection);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 1);
}
