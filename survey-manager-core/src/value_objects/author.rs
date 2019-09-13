use domain_patterns::models::ValueObject;
use crate::value_objects::ValidationError;
use crate::errors::{Error, Result};

#[derive(ValueSetup)]
pub struct Author {
    value: String,
}

impl ValueObject<String> for Author {
    type ValueError = Error;

    fn validate(value: &String) -> Result<()> {
        let min = 3;
        let max = 32;
        let len = value.len();

        if len < min || len > max {
            return Err(
                ValidationError::AuthorsValidationError {
                    msg: format!("length must be between {} and {}", min, max),
                }.into()
            );
        }

        Ok(())
    }

    fn value(&self) -> String {
        return self.value.clone()
    }
}
