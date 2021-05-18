use crate::data::database_pool::DatabasePool;
use crate::data::gurl::Gurl;
use actix_web::{web, HttpResponse, Resource};

pub struct GurlApi {}

impl GurlApi {
    pub fn configure(sc: &mut web::ServiceConfig) {
        sc.service(GurlPath::DELETE.configure())
            .service(GurlPath::GET.configure())
            .service(GurlPath::INSERT.configure());
    }
}

enum GurlPath {
    DELETE,
    GET,
    INSERT,
}

impl GurlPath {
    fn value(&self) -> String {
        let value = match self {
            GurlPath::DELETE => "/gurl/delete/{id_value}",
            GurlPath::GET => "/gurl/get/{url_value}",
            GurlPath::INSERT => "/gurl/insert/{url_value}/{liked}",
        };
        value.to_string()
    }

    fn configure(&self) -> Resource {
        match self {
            GurlPath::DELETE => {
                web::resource(GurlPath::DELETE.value()).route(web::get().to(delete_gurl))
            }
            GurlPath::GET => web::resource(GurlPath::GET.value()).route(web::get().to(get_gurl)),
            GurlPath::INSERT => {
                web::resource(GurlPath::INSERT.value()).route(web::get().to(insert_gurl))
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
