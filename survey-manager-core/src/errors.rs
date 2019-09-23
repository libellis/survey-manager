use snafu::Snafu;
use std::result;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("{}", source))]
    ValidationError {
        source: crate::value_objects::ValidationError,
    },

    /// Represents a resource that has not been found.
    #[snafu(display("resource '{}' was not found", resource))]
    ResourceNotFound {
        resource: String,
    },

    /// NotAuthorized conveys that the caller is not authorized to commit the action.
    #[snafu(display("not authorized"))]
    NotAuthorized,

    /// NotAuthorized conveys that the caller is not authorized to commit the action.
    #[snafu(display("unknown failure occurred"))]
    UnknownFailure,

    /// NotAuthorized conveys that the caller is not authorized to commit the action.
    #[snafu(display("operation failed due to a concurrency error."))]
    ConcurrencyFailure,

    /// RepoFailure conveys to the caller that some kind of error happened in a repository.
    /// This might have been a concurrency error, or failure to communicate with the database.
    #[snafu(display("{}", source))]
    RepoFailure {
        source: Box<dyn std::error::Error + Send>,
    },
}

impl From<crate::value_objects::ValidationError> for Error {
    fn from(err: crate::value_objects::ValidationError) -> Self {
        Error::ValidationError {
            source: err,
        }
    }
}
