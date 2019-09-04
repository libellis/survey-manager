use domain_patterns::models::ValueObject;

#[derive(ValueSetup)]
pub struct Title {
    value: String,
}

impl ValueObject<String> for Title {
    fn validate(value: &String) -> bool {
        value.len() < 128
    }

    fn value(&self) -> String {
        return self.value.clone()
    }
}