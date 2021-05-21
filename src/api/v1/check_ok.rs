use crate::api::v1::paths;
use actix_web::{web, HttpResponse};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource(paths::CHECK_OK).route(web::get().to(check_ok)));
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CheckOk {
    pub message: String,
}

impl CheckOk {
    pub fn default() -> Self {
        Self {
            message: String::from("GuardianServer is running"),
        }
    }
}

pub fn check_ok() -> HttpResponse {
    HttpResponse::Ok().json(CheckOk::default())
}
