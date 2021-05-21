use actix_web::{test, App};
use guardian_backend::api;
use guardian_backend::data::database_pool;
use guardian_backend::data::gurl::Gurl;

#[actix_rt::test]
pub async fn guardian_server_works() {
    dotenv::dotenv().ok();

    let database_pool = database_pool::get("DATABASE_URL");

    let mut guardian_service = test::init_service(
        App::new()
            .data(database_pool.clone())
            .configure(api::v1::gurl::configure),
    )
    .await;

    let struct_get_gurl = api::v1::gurl::GetGurl {
        url: String::from("hello"),
    };
    let request_get = test::TestRequest::get()
        .uri(api::v1::paths::GURL)
        .set_json(&struct_get_gurl)
        .to_request();

    let resonse_get_gurl = test::call_service(&mut guardian_service, request_get).await;
    assert!(resonse_get_gurl.status().is_success());

    let vector_of_gurls: Vec<Gurl> = test::read_body_json(resonse_get_gurl).await;
    assert_eq!(vector_of_gurls.len(), 1);
}
