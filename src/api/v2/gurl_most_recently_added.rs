use crate::api::v2::methods::{gurl_most_recently_added, pooled_connection};
use crate::api::v2::paths;
use crate::data::v2::database::database_pool::DatabasePool;
use actix_web::ResponseError;

pub fn configure(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(
        actix_web::web::resource(paths::GURL_MOST_RECENTLY_ADDED)
            .route(actix_web::web::get().to(handle_request)),
    );
}

async fn handle_request(
    database_pool: actix_web::web::Data<DatabasePool>,
) -> actix_web::HttpResponse {
    match pooled_connection::get(database_pool) {
        Ok(pooled_connection) => match gurl_most_recently_added::get(pooled_connection).await {
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
