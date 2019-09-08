use domain_patterns::models::ValueObject;

#[derive(ValueSetup)]
pub struct Description {
    value: String,
}

impl ValueObject<String> for Description {
    fn validate(value: &String) -> bool {
        value.len() < 256
    }

    fn value(&self) -> String {
        return self.value.clone()
    }
}
