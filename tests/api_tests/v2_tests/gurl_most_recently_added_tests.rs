use actix_web::{test, App};
use guardian_backend::api;
use guardian_backend::data::v1::database_pool;
use guardian_backend::models::v2::gurl::{Gurl, GurlRequest};

#[actix_rt::test]
pub async fn get_gurl_most_recently_added_works() {
    dotenv::dotenv().ok();

    let database_pool = database_pool::get("DATABASE_URL");

    let mut guardian_service = test::init_service(
        App::new()
            .data(database_pool.clone())
            .configure(api::v2::gurl::configure)
            .configure(api::v2::gurl_most_recently_added::configure),
    )
    .await;

    let url_test_value: String = "api_v2_gurl_most_recently_added_works".to_string();
    let liked_test_value: bool = false;

    let gurl_request_insert = GurlRequest {
        id: None,
        url: Some(url_test_value.to_string()),
        liked: Some(liked_test_value),
    };
    let request_insert_one = test::TestRequest::post()
        .uri(api::v2::paths::GURL)
        .set_json(&gurl_request_insert)
        .to_request();
    let _response_insert_one = test::call_service(&mut guardian_service, request_insert_one).await;
    let request_insert_two = test::TestRequest::post()
        .uri(api::v2::paths::GURL)
        .set_json(&gurl_request_insert)
        .to_request();
    let _response_insert_two = test::call_service(&mut guardian_service, request_insert_two).await;

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
    let vector_of_gurls: Vec<Gurl> = test::read_body_json(response_get).await;
    assert_eq!(vector_of_gurls.len(), 2);
    let vector_of_ids: Vec<i32> = vector_of_gurls.iter().map(|gurl| gurl.id).collect();

    let request_get_most_recently_added_gurl = test::TestRequest::get()
        .uri(api::v2::paths::GURL_MOST_RECENTLY_ADDED)
        .to_request();
    let response_get_most_recently_added_gurl =
        test::call_service(&mut guardian_service, request_get_most_recently_added_gurl).await;
    let most_recently_added_gurl: Option<Gurl> =
        test::read_body_json(response_get_most_recently_added_gurl).await;
    assert!(most_recently_added_gurl.is_some());
    assert_eq!(vector_of_ids[1], most_recently_added_gurl.unwrap().id);

    for gurl_id in vector_of_ids {
        let gurl_request_delete = GurlRequest {
            id: Some(gurl_id),
            url: None,
            liked: None,
        };
        let request_delete = test::TestRequest::delete()
            .uri(api::v2::paths::GURL)
            .set_json(&gurl_request_delete)
            .to_request();
        let _response_delete = test::call_service(&mut guardian_service, request_delete).await;
    }
}
