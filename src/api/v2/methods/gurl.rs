use crate::data::v2::methods::gurl;
use crate::models::v2::api_error::ApiError;
use crate::models::v2::gurl::GurlRequest;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::PgConnection;

pub async fn process_gurl_request(
    pooled_connection: PooledConnection<ConnectionManager<PgConnection>>,
    gurl_request: GurlRequest,
) -> Result<actix_web::HttpResponse, ApiError> {
    let (has_id, has_url, has_liked): (bool, bool, bool) = (
        gurl_request.id.is_some(),
        gurl_request.url.is_some(),
        gurl_request.liked.is_some(),
    );
    match (has_id, has_url, has_liked) {
        (true, false, false) => delete(pooled_connection, gurl_request.id.unwrap()).await,
        (false, true, false) => get(pooled_connection, gurl_request.url.unwrap()).await,
        (false, true, true) => {
            insert(
                pooled_connection,
                gurl_request.url.unwrap(),
                gurl_request.liked.unwrap(),
            )
            .await
        }
        _ => Err(ApiError::BadRequest(gurl_request)),
    }
}

async fn delete(
    pooled_connection: PooledConnection<ConnectionManager<PgConnection>>,
    id_value: i32,
) -> Result<actix_web::HttpResponse, ApiError> {
    actix_web::web::block(move || gurl::delete(id_value, &pooled_connection))
        .await
        .map(|number_of_deletions| actix_web::HttpResponse::Ok().json(number_of_deletions))
        .map_err(Into::into)
}

async fn insert(
    pooled_connection: PooledConnection<ConnectionManager<PgConnection>>,
    url_value: String,
    liked: bool,
) -> Result<actix_web::HttpResponse, ApiError> {
    actix_web::web::block(move || gurl::insert(url_value, liked, &pooled_connection))
        .await
        .map(|number_of_insertions| actix_web::HttpResponse::Ok().json(number_of_insertions))
        .map_err(Into::into)
}

async fn get(
    pooled_connection: PooledConnection<ConnectionManager<PgConnection>>,
    url_value: String,
) -> Result<actix_web::HttpResponse, ApiError> {
    actix_web::web::block(move || gurl::get(url_value, &pooled_connection))
        .await
        .map(|gurl_vector| actix_web::HttpResponse::Ok().json(gurl_vector))
        .map_err(Into::into)
}
