#[derive(Debug)]
pub enum DatabasePoolError {
    DatabasePoolBuilder(r2d2::Error),
    DotEnv(dotenv::Error),
}

#[derive(Debug, Serialize)]
struct DatabasePoolErrorResponse {
    error: String,
}

// Display
impl std::fmt::Display for DatabasePoolError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DatabasePoolError::DatabasePoolBuilder(e) => {
                write!(f, "DatabasePoolError::DatabasePoolBuilder {:?}", e)
            }
            DatabasePoolError::DotEnv(e) => {
                write!(f, "DatabasePoolError::DotEnv {:?}", e)
            }
        }
    }
}

// From
impl From<r2d2::Error> for DatabasePoolError {
    fn from(e: r2d2::Error) -> Self {
        DatabasePoolError::DatabasePoolBuilder(e)
    }
}

impl From<dotenv::Error> for DatabasePoolError {
    fn from(e: dotenv::Error) -> Self {
        DatabasePoolError::DotEnv(e)
    }
}

// actix_web::ResponseError
impl actix_web::ResponseError for DatabasePoolError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        let error = format!("{}", self);
        actix_web::HttpResponse::InternalServerError().json(DatabasePoolErrorResponse { error })
    }
}
