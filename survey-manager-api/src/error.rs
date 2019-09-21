use survey_manager_core::Error as SMError;
use actix_web::{ResponseError, HttpResponse, http};
use actix_web::dev::HttpResponseBuilder;

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
    fn render_response(&self) -> HttpResponse {
        let error_struct = ErrorJson::from(self);
        match (*self).0 {
            SMError::RepoFailure {..} => {
                HttpResponseBuilder::new(http::StatusCode::INTERNAL_SERVER_ERROR).json(error_struct)
            }
            SMError::ValidationError {..} => HttpResponseBuilder::new(http::StatusCode::UNPROCESSABLE_ENTITY).json(error_struct),
            SMError::ResourceNotFound {..} => HttpResponseBuilder::new(http::StatusCode::NOT_FOUND).json(error_struct),
            SMError::NotAuthorized => HttpResponseBuilder::new(http::StatusCode::FORBIDDEN).json(error_struct),
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
    #[display(fmt = "Missing bearer token from headers.")]
    MissingBearer,
}


#[derive(Serialize)]
pub struct ErrorJson {
    error: String,
}

impl From<&TokenError> for ErrorJson {
    fn from(err: &TokenError) -> Self {
        ErrorJson {
            error: format!("{}", err),
        }
    }
}

impl From<&Error> for ErrorJson {
    fn from(err: &Error) -> Self {
        ErrorJson {
            error: format!("{}", err),
        }
    }
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
    fn render_response(&self) -> HttpResponse {
        match *self {
            TokenError::MissingBearer => {
                HttpResponseBuilder::new(http::StatusCode::BAD_REQUEST).json(ErrorJson::from(self))
            }
        }
    }
}