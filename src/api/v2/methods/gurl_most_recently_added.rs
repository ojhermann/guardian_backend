use crate::data::v2::methods::gurl;
use crate::models::v2::api_error::ApiError;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::PgConnection;

pub async fn get(
    pooled_connection: PooledConnection<ConnectionManager<PgConnection>>,
) -> Result<actix_web::HttpResponse, ApiError> {
    actix_web::web::block(move || gurl::get_most_recently_added_gurl(&pooled_connection))
        .await
        .map(|gurl_maybe| actix_web::HttpResponse::Ok().json(gurl_maybe))
        .map_err(Into::into)
}
