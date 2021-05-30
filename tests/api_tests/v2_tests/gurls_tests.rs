use actix_web::{test, App};
use guardian_backend::api;
use guardian_backend::data::v1::database_pool;
use guardian_backend::models::v2::gurl::{Gurl, GurlRequest, GurlsRequest};

#[actix_rt::test]
pub async fn get_gurls_works_with_proper_inputs() {
    dotenv::dotenv().ok();

    let database_pool = database_pool::get("DATABASE_URL");

    let mut guardian_service = test::init_service(
        App::new()
            .data(database_pool.clone())
            .configure(api::v2::gurl::configure)
            .configure(api::v2::gurl_most_recently_added::configure)
            .configure(api::v2::gurls::configure),
    )
    .await;

    let url_test_value: String = "get_gurls_works_with_proper_inputs".to_string();
    let liked_test_value: bool = false;

    // insert the first gurl
    let gurl_request_insert = GurlRequest {
        id: None,
        url: Some(url_test_value.to_string()),
        liked: Some(liked_test_value),
    };
    let request_insert = test::TestRequest::post()
        .uri(api::v2::paths::GURL)
        .set_json(&gurl_request_insert)
        .to_request();
    let _response_insert = test::call_service(&mut guardian_service, request_insert).await;

    // get the most recently added gurl and record the start id
    let request_get = test::TestRequest::get()
        .uri(api::v2::paths::GURL_MOST_RECENTLY_ADDED)
        .to_request();
    let response_get = test::call_service(&mut guardian_service, request_get).await;
    assert!(response_get.status().is_success());
    let gurl_maybe: Option<Gurl> = test::read_body_json(response_get).await;
    let start_id = gurl_maybe.unwrap().id;

    // insert 9 more gurls
    for _ in 0..9 {
        let request_insert = test::TestRequest::post()
            .uri(api::v2::paths::GURL)
            .set_json(&gurl_request_insert)
            .to_request();
        let _response_insert = test::call_service(&mut guardian_service, request_insert).await;
    }

    // record the end_id
    let request_get = test::TestRequest::get()
        .uri(api::v2::paths::GURL_MOST_RECENTLY_ADDED)
        .to_request();
    let response_get = test::call_service(&mut guardian_service, request_get).await;
    let gurl_maybe: Option<Gurl> = test::read_body_json(response_get).await;
    let end_id = gurl_maybe.unwrap().id + 1;

    // get all the gurls in the range of ids and confirm there are 10
    let gurls_request_get = GurlsRequest { start_id, end_id };
    let request_get = test::TestRequest::get()
        .uri(api::v2::paths::GURLS)
        .set_json(&gurls_request_get)
        .to_request();
    let response_get = test::call_service(&mut guardian_service, request_get).await;
    let vector_of_gurls: Vec<Gurl> = test::read_body_json(response_get).await;
    assert_eq!(vector_of_gurls.len(), 10);

    // delete these gurls
    for gurl in vector_of_gurls {
        let gurl_request_delete = GurlRequest {
            id: Some(gurl.id),
            url: None,
            liked: None,
        };
        let request_delete = test::TestRequest::delete()
            .uri(api::v2::paths::GURL)
            .set_json(&gurl_request_delete)
            .to_request();
        let _response_delete = test::call_service(&mut guardian_service, request_delete).await;
    }

    // confirm there are no girls left in that range
    let gurls_request_get = GurlsRequest { start_id, end_id };
    let request_get = test::TestRequest::get()
        .uri(api::v2::paths::GURLS)
        .set_json(&gurls_request_get)
        .to_request();
    let response_get = test::call_service(&mut guardian_service, request_get).await;
    let vector_of_gurls: Vec<Gurl> = test::read_body_json(response_get).await;
    assert!(vector_of_gurls.is_empty());
}
