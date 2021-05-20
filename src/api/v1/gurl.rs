use crate::data::database_pool::DatabasePool;
use crate::data::gurl::Gurl;
use crate::data::gurl_error::GurlError;
use actix_web::{web, HttpResponse};

#[derive(Debug, Deserialize, Serialize)]
pub struct DeleteGurl {
    pub id: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetGurl {
    pub url: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct InsertGurl {
    pub url: String,
    pub liked: bool,
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
