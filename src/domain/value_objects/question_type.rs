use domain_patterns::models::ValueObject;
use std::convert::TryFrom;
use crate::errors::{Error, Result};
use crate::errors::ErrorKind::ValidationError;
use crate::domain::value_objects::ValidationError::QuestionTypeValidationError;

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
    type Error = Error;

    fn try_from(value: String) -> Result<Self> {
        Self::validate(&value)?;

        let content_type = match value.as_ref() {
            "ranked" => QuestionType::Ranked,
            "multiple_choice" => QuestionType::MultipleChoice,
            _ => QuestionType::MultipleChoice,
        };

        Ok(content_type)
    }
}

impl ValueObject<String> for QuestionType {
    type ValueError = Error;

    fn validate(value: &String) -> Result<()> {
        match value.as_ref() {
            "multiple_choice" => Ok(()),
            "ranked" => Ok(()),
            _ => Err(
                QuestionTypeValidationError.into()
            ),
        }
    }

    fn value(&self) -> String {
        match self {
            QuestionType::Ranked => "ranked".to_string(),
            QuestionType::MultipleChoice => "multiple_choice".to_string(),
        }
    }
}
