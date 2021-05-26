#[derive(Debug)]
pub enum DatabaseMethodError {
    DieselResult(diesel::result::Error),
}

#[derive(Debug, Serialize)]
struct DatabaseMethodErrorResponse {
    error: String,
}

// Display
impl std::fmt::Display for DatabaseMethodError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DatabaseMethodError::DieselResult(e) => {
                write!(f, "DatabaseMethodError::DieselResult {:?}", e)
            }
        }
    }
}

// From
impl From<diesel::result::Error> for DatabaseMethodError {
    fn from(e: diesel::result::Error) -> Self {
        DatabaseMethodError::DieselResult(e)
    }
}

// actix_web::ResponseError
impl actix_web::ResponseError for DatabaseMethodError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        let error = format!("{}", self);
        actix_web::HttpResponse::InternalServerError().json(DatabaseMethodErrorResponse { error })
    }
}
