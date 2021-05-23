use actix_web::{test, App};
use guardian_backend::api;
use guardian_backend::data::database_pool;
use guardian_backend::models::gurl::Gurl;

#[actix_rt::test]
pub async fn gurl_works() {
    dotenv::dotenv().ok();

    let database_pool = database_pool::get("DATABASE_URL");

    let mut guardian_service = test::init_service(
        App::new()
            .data(database_pool.clone())
            .configure(api::v1::gurl::configure),
    )
    .await;

    let url_test_value: String = "gurl_works".to_string();

    let liked_test_value: bool = false;

    // test insert
    let insert_gurl_test_struct = api::v1::gurl::InsertGurl {
        url: url_test_value.clone(),
        liked: liked_test_value,
    };
    let request_insert_gurl = test::TestRequest::post()
        .uri(api::v1::paths::GURL)
        .set_json(&insert_gurl_test_struct)
        .to_request();
    let response_insert_gurl = test::call_service(&mut guardian_service, request_insert_gurl).await;
    assert!(response_insert_gurl.status().is_success());
    let number_of_insertions: usize = test::read_body_json(response_insert_gurl).await;
    assert_eq!(number_of_insertions, 1);

    // test get with value known to exist
    let get_gurl_test_struct = api::v1::gurl::GetGurl {
        url: url_test_value.clone(),
    };
    let request_get = test::TestRequest::get()
        .uri(api::v1::paths::GURL)
        .set_json(&get_gurl_test_struct)
        .to_request();
    let resonse_get_gurl = test::call_service(&mut guardian_service, request_get).await;
    assert!(resonse_get_gurl.status().is_success());
    let vector_of_gurls: Vec<Gurl> = test::read_body_json(resonse_get_gurl).await;
    assert_eq!(vector_of_gurls.len(), 1);
    assert_eq!(vector_of_gurls[0].url, url_test_value);

    // test delete
    let gurl_id = vector_of_gurls[0].id;
    let delete_gurl_test_struct = api::v1::gurl::DeleteGurl { id: gurl_id };
    let request_delete_gurl = test::TestRequest::delete()
        .uri(api::v1::paths::GURL)
        .set_json(&delete_gurl_test_struct)
        .to_request();
    let response_delete_gurl = test::call_service(&mut guardian_service, request_delete_gurl).await;
    assert!(response_delete_gurl.status().is_success());
    let number_of_deletions: usize = test::read_body_json(response_delete_gurl).await;
    assert_eq!(number_of_deletions, 1);

    // test get with value known not to exists
    let request_get = test::TestRequest::get()
        .uri(api::v1::paths::GURL)
        .set_json(&get_gurl_test_struct)
        .to_request();
    let response_get_gurl = test::call_service(&mut guardian_service, request_get).await;
    assert!(response_get_gurl.status().is_success());
    let vector_of_gurls: Vec<Gurl> = test::read_body_json(response_get_gurl).await;
    assert_eq!(vector_of_gurls.len(), 0);
}
