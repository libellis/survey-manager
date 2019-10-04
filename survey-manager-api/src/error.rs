use survey_manager_core::Error as SMError;
use actix_web::{ResponseError, HttpResponse, http};
use actix_web::dev::HttpResponseBuilder;
use actix_web::error::BlockingError;

#[derive(Debug)]
pub enum ApiError {
    CoreError(CoreError),
    TokenError(TokenError),
    ThreadFailure,
}

impl std::error::Error for ApiError {}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ApiError::CoreError(e) => std::fmt::Display::fmt(e, f),
            ApiError::TokenError(e) => std::fmt::Display::fmt(e, f),
            ApiError::ThreadFailure => {
                write!(f, "Catastrophic thread failure in actix web block.")
            }
        }
    }
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ApiError::CoreError(e) => e.error_response(),
            ApiError::TokenError(e) => e.error_response(),
            ApiError::ThreadFailure => HttpResponse::new(http::StatusCode::INTERNAL_SERVER_ERROR),
        }
    }

    fn render_response(&self) -> HttpResponse {
        match self {
            ApiError::CoreError(e) => e.render_response(),
            ApiError::TokenError(e) => e.render_response(),
            ApiError::ThreadFailure => HttpResponse::new(http::StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}

// We create this here because Rust's orphan rules won't let us impl traits in crates/modules
// where they weren't defined (we can't implement actix error traits inside survey-manager-core
// so we wrap them and implement it on our owned type here)
#[derive(Debug, From)]
pub struct CoreError(pub SMError);

impl std::error::Error for CoreError {}

impl std::fmt::Display for CoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}

impl ResponseError for CoreError {
    fn error_response(&self) -> HttpResponse {
        match (*self).0 {
            SMError::RepoFailure {..} => {
                HttpResponse::new(http::StatusCode::INTERNAL_SERVER_ERROR)
            }
            SMError::ValidationError {..} => HttpResponse::new(http::StatusCode::UNPROCESSABLE_ENTITY),
            SMError::ResourceNotFound {..} => HttpResponse::new(http::StatusCode::NOT_FOUND),
            SMError::NotAuthorized => HttpResponse::new(http::StatusCode::FORBIDDEN),
            SMError::UnknownFailure => HttpResponse::new(http::StatusCode::INTERNAL_SERVER_ERROR),
            SMError::ConcurrencyFailure => HttpResponse::new(http::StatusCode::INTERNAL_SERVER_ERROR),
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
            SMError::UnknownFailure => HttpResponse::new(http::StatusCode::INTERNAL_SERVER_ERROR),
            SMError::ConcurrencyFailure => HttpResponse::new(http::StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}

impl From<BlockingError<SMError>> for ApiError {
    fn from(blocking_err: BlockingError<SMError>) -> Self {
        match blocking_err {
            BlockingError::Error(e) => ApiError::CoreError(CoreError(e)),
            _ => ApiError::ThreadFailure,
        }
    }
}

impl From<CoreError> for ApiError {
    fn from(err: CoreError) -> Self {
        ApiError::CoreError(err)
    }
}

impl From<SMError> for ApiError {
    fn from(err: SMError) -> Self {
        ApiError::CoreError(CoreError(err))
    }
}

#[derive(Debug, Display, From)]
pub enum TokenError {
    #[display(fmt = "Missing bearer token from headers.")]
    MissingBearer,

    #[display(fmt = "Token has expired.")]
    TokenExpired,
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

impl From<&CoreError> for ErrorJson {
    fn from(err: &CoreError) -> Self {
        ErrorJson {
            error: format!("{}", err),
        }
    }
}

impl From<&ApiError> for ErrorJson {
    fn from(err: &ApiError) -> Self {
        ErrorJson {
            error: format!("{}", err)
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
            TokenError::TokenExpired => {
                HttpResponse::new(http::StatusCode::UNAUTHORIZED)
            }
        }
    }
    fn render_response(&self) -> HttpResponse {
        match *self {
            TokenError::MissingBearer => {
                HttpResponseBuilder::new(http::StatusCode::BAD_REQUEST).json(ErrorJson::from(self))
            }
            TokenError::TokenExpired => {
                HttpResponseBuilder::new(http::StatusCode::UNAUTHORIZED).json(ErrorJson::from(self))
            }
        }
    }
}

impl From<BlockingError<TokenError>> for ApiError {
    fn from(blocking_err: BlockingError<TokenError>) -> Self {
        match blocking_err {
            BlockingError::Error(e) => ApiError::TokenError(e),
            BlockingError::Canceled => ApiError::ThreadFailure
        }
    }
}

impl From<TokenError> for ApiError {
    fn from(err: TokenError) -> Self {
        ApiError::TokenError(err)
    }
}
