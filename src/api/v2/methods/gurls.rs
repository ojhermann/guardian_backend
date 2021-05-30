use crate::data::v2::methods::gurl;
use crate::models::v2::api_error::ApiError;
use crate::models::v2::gurl::GurlsRequest;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::PgConnection;

pub async fn get(
    pooled_connection: PooledConnection<ConnectionManager<PgConnection>>,
    gurls_request: GurlsRequest,
) -> Result<actix_web::HttpResponse, ApiError> {
    actix_web::web::block(move || {
        gurl::get_gurls(
            gurls_request.start_id,
            gurls_request.end_id,
            &pooled_connection,
        )
    })
    .await
    .map(|gurl_vec| actix_web::HttpResponse::Ok().json(gurl_vec))
    .map_err(Into::into)
}
