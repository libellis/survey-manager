use domain_patterns::models::ValueObject;
use std::convert::TryFrom;
use crate::value_objects::ValidationError;
use crate::errors::{Error, Result};

#[derive(Clone, PartialEq)]
pub enum ContentType {
    Text,
    Youtube,
    Spotify,
    Soundcloud,
}

impl std::fmt::Display for ContentType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.value())
    }
}

impl TryFrom<String> for ContentType {
    type Error = Error;

    fn try_from(value: String) -> Result<Self> {
        Self::validate(&value)?;

        let content_type = match value.as_ref() {
            "text" => ContentType::Text,
            "youtube" => ContentType::Youtube,
            "spotify" => ContentType::Spotify,
            "soundcloud" => ContentType::Soundcloud,
            _ => ContentType::Text,
        };

        Ok(content_type)
    }
}

impl ValueObject<String> for ContentType {
    type ValueError = Error;

    fn validate(value: &String) -> Result<()> {
        match value.as_ref() {
            "text" => Ok(()),
            "youtube" => Ok(()),
            "spotify" => Ok(()),
            "soundcloud" => Ok(()),
            _ => Err(
                ValidationError::ContentTypeValidationError.into()
            ),
        }
    }

    fn value(&self) -> String {
        match self {
            ContentType::Text => "text".to_string(),
            ContentType::Spotify => "spotify".to_string(),
            ContentType::Soundcloud => "soundcloud".to_string(),
            ContentType::Youtube => "youtube".to_string(),
        }
    }
}
