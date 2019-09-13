use domain_patterns::models::ValueObject;
use std::convert::TryFrom;
use crate::value_objects::ValidationError;
use crate::errors::{Error, Result};

#[derive(Clone, PartialEq)]
pub enum Category {
    Music,
    Funny,
    Technology,
    Memes,
}

impl std::fmt::Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.value())
    }
}

impl TryFrom<String> for Category {
    type Error = Error;

    fn try_from(value: String) -> Result<Self> {
        Self::validate(&value)?;

        let content_type = match value.as_ref() {
            "music" => Category::Music,
            "funny" => Category::Funny,
            "technology" => Category::Technology,
            "memes" => Category::Memes,
            _ => Category::Music,
        };

        Ok(content_type)
    }
}

impl ValueObject<String> for Category {
    type ValueError = Error;

    fn validate(value: &String) -> Result<()> {
        match value.as_ref() {
            "music" => Ok(()),
            "technology" => Ok(()),
            "memes" => Ok(()),
            "funny" => Ok(()),
            _ => Err(
                ValidationError::CategoryValidationError {
                    msg: "This is not a valid category.".to_string()
                }.into()
            ),
        }
    }

    fn value(&self) -> String {
        match self {
            Category::Music => "music".to_string(),
            Category::Technology => "technology".to_string(),
            Category::Memes => "memes".to_string(),
            Category::Funny => "funny".to_string(),
        }
    }
}
