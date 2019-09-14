use failure::{Context, Fail, Backtrace};
use crate::value_objects::ValidationError;
use std::fmt;
use std::result;

pub type Result<T> = result::Result<T, Error>;

// An error that can occur while using the survey manager.
#[derive(Debug)]
pub struct Error {
    ctx: Context<ErrorKind>,
}

impl Fail for Error {
    fn cause(&self) -> Option<&dyn Fail> {
        self.ctx.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.ctx.backtrace()
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt::Display::fmt(&self.ctx, f)
    }
}

impl Error {
    /// Return the kind of this error.
    pub fn kind(&self) -> &ErrorKind {
        self.ctx.get_context()
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum ErrorKind {
    ValidationError(ValidationError),

    /// Represents a resource that has not been found.
    ResourceNotFound(String),

    /// NotAuthorized conveys that the caller is not authorized to commit the action.
    NotAuthorized,

    // TODO: Flush this out.  We work with a generic repository so we can't possibly know in advance
    // what kind of database level errors it might return.  How do we make this better?
    /// DbFailure conveys to the caller that some kind of error happened on the database level.
    /// This might have been a concurrency error, or failure to communicate with the database.
    DbFailure,

    /// Hints that destructuring should not be exhaustive.
    ///
    /// This enum may grow additional variants, so this makes sure clients
    /// don't count on exhaustive matching. (Otherwise, adding a new variant
    /// could break existing code.)
    #[doc(hidden)]
    __Nonexhaustive,
}

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ErrorKind::ValidationError(v) => {
                std::fmt::Display::fmt(v, f)
            }
            ErrorKind::ResourceNotFound(ref name) => {
                write!(f, "resource '{}' was not found", name)
            }
            ErrorKind::NotAuthorized => {
                write!(f, "not Authorized")
            }
            ErrorKind::DbFailure => {
                write!(f, "database failure")
            }
            ErrorKind::__Nonexhaustive => panic!("invalid error"),
        }
    }
}

impl From<ValidationError> for ErrorKind {
    fn from(inner: ValidationError) -> Self {
        ErrorKind::ValidationError(inner)
    }
}

impl From<ValidationError> for Error {
    fn from(inner: ValidationError) -> Self {
        Error::from(Context::new(ErrorKind::from(inner)))
    }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Error {
        Error::from(Context::new(kind))
    }
}

impl From<Context<ErrorKind>> for Error {
    fn from(ctx: Context<ErrorKind>) -> Error {
        Error { ctx }
    }
}
