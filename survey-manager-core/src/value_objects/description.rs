use domain_patterns::models::ValueObject;
use crate::errors::{Error, Result};
use crate::value_objects::ValidationError;

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
                ValidationError::DescriptionValidationError {
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
