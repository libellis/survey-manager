use domain_patterns::models::ValueObject;
use std::convert::TryFrom;

#[derive(Clone, PartialEq)]
pub enum QuestionType {
    Ranked,
    MultipleChoice,
}

impl std::fmt::Display for QuestionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.value())
    }
}

impl TryFrom<String> for QuestionType {
    type Error = QuestionTypeValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if Self::validate(&value) {
            // we've already validated so we can assume we
            // have no edge cases and can set wild to anything.
            let content_type = match value.as_ref() {
                "ranked" => QuestionType::Ranked,
                "multiple_choice" => QuestionType::MultipleChoice,
                _ => QuestionType::MultipleChoice,
            };
            return Ok(content_type);
        }

        Err(QuestionTypeValidationError)
    }
}

#[derive(Debug)]
pub struct QuestionTypeValidationError;

impl std::fmt::Display for QuestionTypeValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "QuestionType failed to validate.")
    }
}

impl std::error::Error for QuestionTypeValidationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl ValueObject<String> for QuestionType {
    fn validate(value: &String) -> bool {
        match value.as_ref() {
            "multiple_choice" => true,
            "ranked" => true,
            _ => false,
        }
    }

    fn value(&self) -> String {
        match self {
            QuestionType::Ranked => "ranked".to_string(),
            QuestionType::MultipleChoice => "multiple_choice".to_string(),
        }
    }
}
