use domain_patterns::models::ValueObject;
use std::convert::TryFrom;

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
    type Error = CategoryValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if Self::validate(&value) {
            // we've already validated so we can assume we
            // have no edge cases and can set wild to anything.
            let content_type = match value.as_ref() {
                "music" => Category::Music,
                "funny" => Category::Funny,
                "technology" => Category::Technology,
                "memes" => Category::Memes,
                _ => Category::Music,
            };
            return Ok(content_type);
        }

        Err(CategoryValidationError)
    }
}

#[derive(Debug)]
pub struct CategoryValidationError;

impl std::fmt::Display for CategoryValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Category failed to validate.")
    }
}

impl std::error::Error for CategoryValidationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl ValueObject<String> for Category {
    fn validate(value: &String) -> bool {
        match value.as_ref() {
            "music" => true,
            "technology" => true,
            "memes" => true,
            "funny" => true,
            _ => false,
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
