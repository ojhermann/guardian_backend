use actix_web::{test, App};
use guardian_backend::api;
use guardian_backend::data::database_pool;
use guardian_backend::data::gurl::Gurl;

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

    let url_for_gurl: String = "gurl_works".to_string();
    let liked: bool = false;

    let struct_insert_gurl = api::v1::gurl::InsertGurl {
        url: url_for_gurl.clone(),
        liked,
    };
    let request_insert_gurl = test::TestRequest::post()
        .uri(api::v1::paths::GURL)
        .set_json(&struct_insert_gurl)
        .to_request();
    let response_insert_gurl = test::call_service(&mut guardian_service, request_insert_gurl).await;
    assert!(response_insert_gurl.status().is_success());
    let number_of_insertions: usize = test::read_body_json(response_insert_gurl).await;
    assert_eq!(number_of_insertions, 1);

    let struct_get_gurl = api::v1::gurl::GetGurl {
        url: url_for_gurl.clone(),
    };
    let request_get = test::TestRequest::get()
        .uri(api::v1::paths::GURL)
        .set_json(&struct_get_gurl)
        .to_request();
    let resonse_get_gurl = test::call_service(&mut guardian_service, request_get).await;
    assert!(resonse_get_gurl.status().is_success());
    let vector_of_gurls: Vec<Gurl> = test::read_body_json(resonse_get_gurl).await;
    assert_eq!(vector_of_gurls.len(), 1);

    let gurl_id = vector_of_gurls[0].id;
    let struct_delete_gurl = api::v1::gurl::DeleteGurl { id: gurl_id };
    let request_delete_gurl = test::TestRequest::delete()
        .uri(api::v1::paths::GURL)
        .set_json(&struct_delete_gurl)
        .to_request();
    let response_delete_gurl = test::call_service(&mut guardian_service, request_delete_gurl).await;
    assert!(response_delete_gurl.status().is_success());
    let number_of_deletions: usize = test::read_body_json(response_delete_gurl).await;
    assert_eq!(number_of_deletions, 1);

    let request_get = test::TestRequest::get()
        .uri(api::v1::paths::GURL)
        .set_json(&struct_get_gurl)
        .to_request();
    let resonse_get_gurl = test::call_service(&mut guardian_service, request_get).await;
    assert!(resonse_get_gurl.status().is_success());
    let vector_of_gurls: Vec<Gurl> = test::read_body_json(resonse_get_gurl).await;
    assert_eq!(vector_of_gurls.len(), 0);
}

// todo create insert, get, delete, and get tests to run as one
