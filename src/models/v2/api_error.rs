use crate::models::v2::database_method_error::DatabaseMethodError;
use crate::models::v2::gurl::GurlRequest;

#[derive(Debug)]
pub enum ApiError {
    BadRequest(GurlRequest),
    Blocking(actix_web::error::BlockingError<DatabaseMethodError>),
}

#[derive(Debug, Serialize)]
struct ApiErrorResponse {
    error: String,
}

// Display
impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiError::BadRequest(request) => write!(f, "ApiError::BadRequest {:?}", request),
            ApiError::Blocking(e) => {
                write!(f, "ApiError::Blocking {:?}", e)
            }
        }
    }
}

// From
impl From<actix_web::error::BlockingError<DatabaseMethodError>> for ApiError {
    fn from(e: actix_web::error::BlockingError<DatabaseMethodError>) -> Self {
        ApiError::Blocking(e)
    }
}

// actix_web::ResponseError
impl actix_web::ResponseError for ApiError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        let error = format!("{}", self);
        actix_web::HttpResponse::InternalServerError().json(ApiErrorResponse { error })
    }
}
