use guardian_backend::data::v1::database_pool::{self, DatabasePool};
use guardian_backend::models::v1::gurl::Gurl;

#[test]
pub fn insert_get_and_delete_work() {
    dotenv::dotenv().ok();

    let database_url_key = "DATABASE_URL";
    let url_value = "insert_get_and_delete_work";
    let dp = database_pool::get(database_url_key);

    insert_works(&dp, url_value);
    insert_works(&dp, url_value);

    let id_values = get_works(&dp, url_value, 2);

    id_values
        .iter()
        .for_each(|id_value| delete_works(&dp, *id_value));

    get_works(&dp, url_value, 0);
}

#[test]
pub fn get_most_recently_added_gurl_works() {
    dotenv::dotenv().ok();

    let database_url_key = "DATABASE_URL";
    let url_value = "get_most_recently_added_gurl_works";
    let dp = database_pool::get(database_url_key);

    let pooled_connection = dp.get().unwrap();
    let _insert_result = Gurl::insert(url_value.to_string(), true, &pooled_connection);

    let get_query_result = Gurl::get(url_value.to_string(), &pooled_connection);
    assert!(get_query_result.is_ok());
    let vec_of_gurls = get_query_result.unwrap();
    assert_eq!(vec_of_gurls.len(), 1);
    let id_of_most_recently_added_gurl = vec_of_gurls.get(0).unwrap().id;

    let tested_fnc_query_result = Gurl::get_most_recently_added_gurl(&pooled_connection);
    assert!(tested_fnc_query_result.is_ok());
    let vec_of_results = tested_fnc_query_result.unwrap();
    assert_eq!(vec_of_results.len(), 1);
    let id_of_tested_query = vec_of_results.get(0).unwrap().id;
    assert_eq!(id_of_most_recently_added_gurl, id_of_tested_query);

    let _delete_result = Gurl::delete(id_of_most_recently_added_gurl, &pooled_connection);
}

fn insert_works(dp: &DatabasePool, url_value: &str) {
    let pooled_connection = dp.get().unwrap();
    let insert_result = Gurl::insert(url_value.to_string(), true, &pooled_connection);
    assert!(insert_result.is_ok());
    assert_eq!(insert_result.unwrap(), 1);
}

fn get_works(dp: &DatabasePool, url_value: &str, expected_size: usize) -> Vec<i32> {
    let pooled_connection = dp.get().unwrap();
    let query_result = Gurl::get(url_value.to_string(), &pooled_connection);

    assert!(query_result.is_ok());

    let results: Vec<Gurl> = query_result.unwrap();
    assert_eq!(results.len(), expected_size);

    results
        .iter()
        .for_each(|gurl| assert_eq!(gurl.url, url_value));

    results.iter().map(|gurl| gurl.id).collect()
}

fn delete_works(dp: &DatabasePool, id_value: i32) {
    let pooled_connection = dp.get().unwrap();
    let delete_result_one = Gurl::delete(id_value, &pooled_connection);
    assert!(delete_result_one.is_ok());
    assert_eq!(delete_result_one.unwrap(), 1);
}
