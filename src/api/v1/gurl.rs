use crate::data::database_pool::DatabasePool;
use crate::data::gurl::Gurl;
use actix_web::{web, HttpResponse, Resource};

pub struct GurlApi {}

impl GurlApi {
    pub fn configure(cfg: &mut web::ServiceConfig) {
        cfg.service(
            web::resource("/gurl/{value}")
                .route(web::get().to(get_gurl))
                .route(web::delete().to(delete_gurl)),
        )
        .service(web::resource("/gurl/{value}/{liked}").route(web::post().to(insert_gurl)));

        // cfg.service(GurlPath::Delete.configure())
        //     .service(GurlPath::Get.configure())
        //     .service(GurlPath::Insert.configure());
    }
}

enum GurlPath {
    Delete,
    Get,
    Insert,
}

impl GurlPath {
    fn value(&self) -> String {
        let value = match self {
            GurlPath::Delete => "/gurl/delete/{id_id}",
            GurlPath::Get => "/gurl/get/{url_value}",
            GurlPath::Insert => "/gurl/insert/{url_value}/{liked}",
        };
        value.to_string()
    }

    fn configure(&self) -> Resource {
        match self {
            GurlPath::Delete => {
                web::resource(GurlPath::Delete.value()).route(web::delete().to(delete_gurl))
            }
            GurlPath::Get => web::resource(GurlPath::Get.value()).route(web::get().to(get_gurl)),
            GurlPath::Insert => {
                web::resource(GurlPath::Insert.value()).route(web::post().to(insert_gurl))
            }
        }
    }
}

pub async fn delete_gurl(
    database_pool: web::Data<DatabasePool>,
    id_value: web::Path<i32>,
) -> HttpResponse {
    let id_value = id_value.into_inner();

    let pooled_connection = database_pool
        .get()
        .expect("get_gurl: could not obtain a connection from the pool");

    let delete_result = web::block(move || Gurl::delete(id_value, &pooled_connection)).await;

    match delete_result {
        Ok(number_of_deletions) => HttpResponse::Ok().json(number_of_deletions),
        Err(_e) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn get_gurl(
    database_pool: web::Data<DatabasePool>,
    url_value: web::Path<String>,
) -> HttpResponse {
    let url_value = url_value.into_inner();

    let pooled_connection = database_pool
        .get()
        .expect("get_gurl: could not obtain a connection from the pool");

    let gurls_result = web::block(move || Gurl::get(url_value, &pooled_connection)).await;

    match gurls_result {
        Ok(vector_of_gurls) => HttpResponse::Ok().json(vector_of_gurls),
        Err(_e) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn insert_gurl(
    database_pool: actix_web::web::Data<DatabasePool>,
    path_params: web::Path<(String, bool)>,
) -> HttpResponse {
    let path_params = path_params.into_inner();
    let url_value = path_params.0;
    let liked = path_params.1;

    let pooled_connection = database_pool
        .get()
        .expect("insert_gurl: could not obtain a connection from the pool");

    let insert_result =
        web::block(move || Gurl::insert(url_value, liked, &pooled_connection)).await;

    match insert_result {
        Ok(number_of_inserts) => HttpResponse::Ok().json(number_of_inserts),
        Err(_e) => HttpResponse::InternalServerError().finish(),
    }
}
