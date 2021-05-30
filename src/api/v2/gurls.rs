use crate::api::v2::methods::{gurls, pooled_connection};
use crate::api::v2::paths;
use crate::data::v2::database::database_pool::DatabasePool;
use crate::models::v2::gurl::GurlsRequest;
use actix_web::ResponseError;

pub fn configure(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(
        actix_web::web::resource(paths::GURLS).route(actix_web::web::get().to(handle_request)),
    );
}

async fn handle_request(
    database_pool: actix_web::web::Data<DatabasePool>,
    json: actix_web::web::Json<GurlsRequest>,
) -> actix_web::HttpResponse {
    match pooled_connection::get(database_pool) {
        Ok(pooled_connection) => match gurls::get(pooled_connection, json.into_inner()).await {
            Ok(response) => response,
            Err(api_error) => {
                log::error!("{}", api_error);
                api_error.error_response()
            }
        },
        Err(database_method_error) => {
            log::error!("{}", database_method_error);
            database_method_error.error_response()
        }
    }
}
