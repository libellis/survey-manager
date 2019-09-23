use snafu::Snafu;

/// ValidationErrors are errors related to failure to validate during creation of a value object.
#[derive(Debug, Snafu)]
pub enum ValidationError {
    #[snafu(display("Author failed to validate. {}", msg))]
    AuthorsValidationError {
        msg: String,
    },
    #[snafu(display("Title failed to validate. {}", msg))]
    TitleValidationError {
        msg: String,
    },
    #[snafu(display("Description failed to validate. {}", msg))]
    DescriptionValidationError {
        msg: String,
    },

    #[snafu(display("Not a valid category."))]
    CategoryValidationError,

    #[snafu(display("Not a valid content type."))]
    ContentTypeValidationError,

    #[snafu(display("Not a valid question type."))]
    QuestionTypeValidationError,
}
