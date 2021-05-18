use actix_web::body::Body;
use actix_web::error::BlockingError;
use actix_web::{App, HttpResponse, HttpServer};
use guardian_backend::data::database_pool;
use guardian_backend::data::database_pool::DatabasePool;
use guardian_backend::data::gurl::Gurl;
use guardian_backend::data::gurl_error::GurlError;
use std::fmt::{Debug, Error};
use std::io;

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv::dotenv().ok();

    let dp = database_pool::get("DATABASE_URL");

    HttpServer::new(move || {
        App::new()
            .data(dp.clone())
            .route("/{url_value}", actix_web::web::get().to(get_gurl))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

async fn get_gurl(
    dp: actix_web::web::Data<DatabasePool>,
    url_value: actix_web::web::Path<String>,
) -> HttpResponse {
    let url_value = url_value.into_inner();
    let conn = dp.get().expect("Couldn't get a connection from the pool");
    let gurl = actix_web::web::block(move || Gurl::get(url_value, &conn)).await;
    match gurl {
        Ok(vg) => HttpResponse::Ok().json(vg),
        Err(_e) => HttpResponse::InternalServerError().finish(),
    }
}

fn convert<T, E>(result: Result<T, E>) -> Result<HttpResponse, GurlError<E>>
where
    T: serde::Serialize,
    E: Debug,
    GurlError<E>: From<E>,
{
    result
        .map(|d| HttpResponse::Ok().json(d))
        .map_err(Into::into)
}
