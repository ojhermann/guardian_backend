use actix_web::{test, App};
use guardian_backend::api;
use guardian_backend::data::database_pool;

#[actix_rt::test]
pub async fn check_ok_works() {
    dotenv::dotenv().ok();

    let database_pool = database_pool::get("DATABASE_URL");

    let mut guardian_service = test::init_service(
        App::new()
            .data(database_pool.clone())
            .configure(api::v1::check_ok::configure),
    )
    .await;

    let request = test::TestRequest::get()
        .uri(api::v1::paths::CHECK_OK)
        .to_request();

    let response = test::call_service(&mut guardian_service, request).await;

    assert!(response.status().is_success());
}
