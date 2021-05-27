use crate::api::v2::methods::gurl;
use crate::data::v2::database::database_pool::DatabasePool;
use crate::models::v2::database_method_error::DatabaseMethodError;
use crate::models::v2::gurl::GurlRequest;
use actix_web::ResponseError;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::PgConnection;

pub async fn handle_gurl_request(
    database_pool: actix_web::web::Data<DatabasePool>,
    json: actix_web::web::Json<GurlRequest>,
) -> actix_web::HttpResponse {
    match get_pooled_connection(database_pool) {
        Ok(pooled_connection) => {
            let gurl_request = json.into_inner();
            match gurl::process_gurl_request(pooled_connection, gurl_request).await {
                Ok(response) => response,
                Err(api_error) => {
                    log::error!("{}", api_error);
                    api_error.error_response()
                }
            }
        }
        Err(database_method_error) => database_method_error.error_response(),
    }
}

fn get_pooled_connection(
    database_pool: actix_web::web::Data<DatabasePool>,
) -> Result<PooledConnection<ConnectionManager<PgConnection>>, DatabaseMethodError> {
    database_pool.get().map_err(Into::into)
}
