use survey_manager_core::Error as SMError;
use actix_web::{ResponseError, HttpResponse, http};

// We create this here because Rust's orphan rules won't let us impl traits in crates/modules
// where they weren't defined (we can't implement actix error traits inside survey-manager-core
// so we wrap them and implement it on our owned type here)
#[derive(Debug)]
pub struct Error(pub SMError);

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        match (*self).0 {
            SMError::RepoFailure {..} => {
                HttpResponse::new(http::StatusCode::INTERNAL_SERVER_ERROR)
            }
            SMError::ValidationError {..} => HttpResponse::new(http::StatusCode::UNPROCESSABLE_ENTITY),
            SMError::ResourceNotFound {..} => HttpResponse::new(http::StatusCode::NOT_FOUND),
            SMError::NotAuthorized => HttpResponse::new(http::StatusCode::FORBIDDEN),
        }
    }
}

impl From<SMError> for Error {
    fn from(error: SMError) -> Self {
        Error(error)
    }
}

#[derive(Debug, Display, From)]
pub enum TokenError {
    #[display(fmt = "Missing Bearer Token")]
    MissingBearer,
}

/// Return `BadRequest` for `TokenError` if missing Bearer Token.
impl ResponseError for TokenError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            TokenError::MissingBearer => {
                HttpResponse::new(http::StatusCode::BAD_REQUEST)
            }
        }
    }
}