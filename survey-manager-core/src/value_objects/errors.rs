/// ValidationErrors are errors related to failure to validate during creation of a value object.
#[derive(Clone, Eq, PartialEq, Debug, Fail)]
pub enum ValidationError {
    #[fail(display = "Author failed to validate. {}", msg)]
    AuthorsValidationError {
        msg: String,
    },
    #[fail(display = "Title failed to validate. {}", msg)]
    TitleValidationError {
        msg: String,
    },
    #[fail(display = "Description failed to validate. {}", msg)]
    DescriptionValidationError {
        msg: String,
    },
    #[fail(display = "Category failed to validate. {}", msg)]
    CategoryValidationError {
        msg: String,
    },
    #[fail(display = "Content type failed to validate. {}", msg)]
    ContentTypeValidationError {
        msg: String,
    },
    #[fail(display = "Not a valid question type.")]
    QuestionTypeValidationError,
}
