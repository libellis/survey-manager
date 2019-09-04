use domain_patterns::models::ValueObject;
use std::convert::TryFrom;

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
    type Error = ContentTypeValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if Self::validate(&value) {
            // we've already validated so we can assume we
            // have no edge cases and can set wild to anything.
            let content_type = match value.as_ref() {
                "text" => ContentType::Text,
                "youtube" => ContentType::Youtube,
                "spotify" => ContentType::Spotify,
                "soundcloud" => ContentType::Soundcloud,
                _ => ContentType::Text,
            };
            return Ok(content_type);
        }

        Err(ContentTypeValidationError)
    }
}

#[derive(Debug)]
pub struct ContentTypeValidationError;

impl std::fmt::Display for ContentTypeValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "ContentType failed to validate.")
    }
}

impl std::error::Error for ContentTypeValidationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl ValueObject<String> for ContentType {
    fn validate(value: &String) -> bool {
        match value.as_ref() {
            "text" => true,
            "youtube" => true,
            "spotify" => true,
            "soundcloud" => true,
            _ => false,
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
