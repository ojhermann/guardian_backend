use crate::api::v1::paths;
use crate::data::v1::database_pool::DatabasePool;
use crate::models::v1::gurl::{DeleteGurl, GetGurl, Gurl, InsertGurl};
use crate::models::v1::gurl_error::GurlError;
use actix_web::{web, HttpResponse};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource(paths::GURL)
            .route(web::delete().to(delete_gurl))
            .route(web::get().to(get_gurl))
            .route(web::post().to(insert_gurl)),
    )
    .service(
        web::resource(paths::GURL_MOST_RECENTLY_ADDED)
            .route(web::get().to(get_most_recently_added_gurl)),
    );
}

pub async fn delete_gurl(
    database_pool: web::Data<DatabasePool>,
    json: web::Json<DeleteGurl>,
) -> HttpResponse {
    let id_value = json.into_inner().id;

    match database_pool.get() {
        Ok(pooled_connection) => {
            match web::block(move || Gurl::delete(id_value, &pooled_connection)).await {
                Ok(number_of_deletions) => HttpResponse::Ok().json(number_of_deletions),
                Err(e) => {
                    log::error!("{}", GurlError::BlockingError(e));
                    HttpResponse::InternalServerError().finish()
                }
            }
        }
        Err(e) => {
            log::error!("delete_gurl: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn get_gurl(
    database_pool: web::Data<DatabasePool>,
    json: web::Json<GetGurl>,
) -> HttpResponse {
    let url = json.into_inner().url;

    match database_pool.get() {
        Ok(pooled_connection) => {
            match web::block(move || Gurl::get(url, &pooled_connection)).await {
                Ok(vector_of_gurls) => HttpResponse::Ok().json(vector_of_gurls),
                Err(e) => {
                    log::error!("get_gurl: {}", GurlError::BlockingError(e));
                    HttpResponse::InternalServerError().finish()
                }
            }
        }
        Err(e) => {
            log::error!("get_gurl: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn get_most_recently_added_gurl(database_pool: web::Data<DatabasePool>) -> HttpResponse {
    match database_pool.get() {
        Ok(pooled_connection) => {
            match web::block(move || Gurl::get_most_recently_added_gurl(&pooled_connection)).await {
                Ok(vector_of_gurls) => HttpResponse::Ok().json(vector_of_gurls.get(0)),
                Err(e) => {
                    log::error!(
                        "get_most_recently_added_gurl: {}",
                        GurlError::BlockingError(e)
                    );
                    HttpResponse::InternalServerError().finish()
                }
            }
        }
        Err(e) => {
            log::error!("get_most_recently_added_gurl: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn insert_gurl(
    database_pool: actix_web::web::Data<DatabasePool>,
    json: web::Json<InsertGurl>,
) -> HttpResponse {
    let insert_gurl = json.into_inner();
    let url_value = insert_gurl.url;
    let liked = insert_gurl.liked;

    match database_pool.get() {
        Ok(pooled_connection) => {
            match web::block(move || Gurl::insert(url_value, liked, &pooled_connection)).await {
                Ok(number_of_inserts) => HttpResponse::Created().json(number_of_inserts),
                Err(e) => {
                    log::error!("insert_gurl: {}", GurlError::BlockingError(e));
                    HttpResponse::InternalServerError().finish()
                }
            }
        }
        Err(e) => {
            log::error!("insert_gurl: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
