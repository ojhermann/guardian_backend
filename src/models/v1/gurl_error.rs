use actix_web::error::BlockingError;
use std::fmt::Debug;

#[derive(Debug)]
pub enum GurlError<E>
where
    E: Debug,
{
    BlockingError(actix_web::error::BlockingError<E>),
    DatabaseError(diesel::result::Error),
}

#[derive(Debug, Serialize)]
struct GurlErrorResponse {
    error: String,
}

impl<E> std::fmt::Display for GurlError<E>
where
    E: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            GurlError::BlockingError(e) => write!(f, "GurlError::BlockingError: {:?}", e),
            GurlError::DatabaseError(e) => write!(f, "GurlError::DatabaseError: {:?}", e),
        }
    }
}

impl<E> From<actix_web::error::BlockingError<E>> for GurlError<E>
where
    E: Debug,
{
    fn from(e: BlockingError<E>) -> Self {
        GurlError::BlockingError(e)
    }
}

impl<E> From<diesel::result::Error> for GurlError<E>
where
    E: Debug,
{
    fn from(e: diesel::result::Error) -> Self {
        GurlError::DatabaseError(e)
    }
}

impl<E> actix_web::ResponseError for GurlError<E>
where
    E: Debug,
{
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            GurlError::BlockingError(_e) => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            GurlError::DatabaseError(_e) => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        let error = format!("{}", self);

        let mut builder = match self {
            GurlError::BlockingError(_e) => actix_web::HttpResponse::InternalServerError(),
            GurlError::DatabaseError(_e) => actix_web::HttpResponse::InternalServerError(),
        };

        builder.json(GurlErrorResponse { error })
    }
}
