use actix_web::{test, App};
use guardian_backend::api;
use guardian_backend::data::v1::database_pool;
use guardian_backend::models::v2::gurl::{Gurl, GurlRequest};

#[actix_rt::test]
pub async fn gurl_works_with_proper_inputs() {
    dotenv::dotenv().ok();

    let database_pool = database_pool::get("DATABASE_URL");

    let mut guardian_service = test::init_service(
        App::new()
            .data(database_pool.clone())
            .configure(api::v2::gurl::configure),
    )
    .await;

    let url_test_value: String = "api_v2_gurl_works".to_string();
    let liked_test_value: bool = false;

    // insert
    let gurl_request_insert = GurlRequest {
        id: None,
        url: Some(url_test_value.to_string()),
        liked: Some(liked_test_value),
    };
    let request_insert = test::TestRequest::post()
        .uri(api::v2::paths::GURL)
        .set_json(&gurl_request_insert)
        .to_request();
    let response_insert = test::call_service(&mut guardian_service, request_insert).await;
    assert!(response_insert.status().is_success());
    let number_of_insertions: usize = test::read_body_json(response_insert).await;
    assert_eq!(number_of_insertions, 1);

    // get
    let gurl_request_get = GurlRequest {
        id: None,
        url: Some(url_test_value.to_string()),
        liked: None,
    };
    let request_get = test::TestRequest::get()
        .uri(api::v2::paths::GURL)
        .set_json(&gurl_request_get)
        .to_request();
    let response_get = test::call_service(&mut guardian_service, request_get).await;
    assert!(response_get.status().is_success());
    let vector_of_gurls: Vec<Gurl> = test::read_body_json(response_get).await;
    assert_eq!(vector_of_gurls.len(), 1);
    assert_eq!(vector_of_gurls[0].url, url_test_value);

    // delete
    let gurl_id = vector_of_gurls[0].id;
    let gurl_request_delete = GurlRequest {
        id: Some(gurl_id),
        url: None,
        liked: None,
    };
    let request_delete = test::TestRequest::delete()
        .uri(api::v2::paths::GURL)
        .set_json(&gurl_request_delete)
        .to_request();
    let response_delete = test::call_service(&mut guardian_service, request_delete).await;
    assert!(response_delete.status().is_success());
    let number_of_deletions: usize = test::read_body_json(response_delete).await;
    assert_eq!(number_of_deletions, 1);

    // get to confirm delete
    let request_get_confirm_delete = test::TestRequest::get()
        .uri(api::v2::paths::GURL)
        .set_json(&gurl_request_get)
        .to_request();
    let response_get_confirm_delete =
        test::call_service(&mut guardian_service, request_get_confirm_delete).await;
    assert!(response_get_confirm_delete.status().is_success());
    let vector_of_gurls: Vec<Gurl> = test::read_body_json(response_get_confirm_delete).await;
    assert_eq!(vector_of_gurls.len(), 0);
}
