use domain_patterns::models::ValueObject;
use crate::errors::{Error, Result};
use crate::domain::value_objects::ValidationError::DescriptionValidationError;

#[derive(ValueSetup)]
pub struct Description {
    value: String,
}

impl ValueObject<String> for Description {
    type ValueError = Error;

    fn validate(value: &String) -> Result<()> {
        let min = 20;
        let max = 256;
        let len = value.len();

        if len < min || len > max {
            return Err(
                DescriptionValidationError {
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
