use domain_patterns::models::ValueObject;
use crate::errors::{Error,Result};
use crate::value_objects::ValidationError;

#[derive(ValueSetup)]
pub struct Title {
    value: String,
}

impl ValueObject<String> for Title {
    type ValueError = Error;

    fn validate(value: &String) -> Result<()> {
        let min = 8;
        let max = 128;
        let len = value.len();

        if len < min || len > max {
            return Err(
                ValidationError::TitleValidationError {
                    msg: format!("Length must be between {} and {}.", min, max),
                }.into()
            );
        }

        Ok(())
    }

    fn value(&self) -> String {
        return self.value.clone()
    }
}