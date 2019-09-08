use domain_patterns::models::ValueObject;

#[derive(ValueSetup)]
pub struct Author {
    value: String,
}

impl ValueObject<String> for Author {
    fn validate(value: &String) -> bool {
        value.len() < 32
    }

    fn value(&self) -> String {
        return self.value.clone()
    }
}
