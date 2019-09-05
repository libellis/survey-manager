use domain_patterns::collections::Repository;
use crate::domain::survey::Survey;
use crate::application::ports::inputs::NewSurveyData;
use std::error::Error;
use std::convert::Into;

pub struct SurveyService<T> where
    T: Repository<Survey>
{
    repo: T,
}

impl<T: Repository<Survey>> SurveyService<T> {

    // Creates a survey by taking in data which is a json string, a token
    // and finally returns a json string as output.
    pub fn create_survey(&mut self, data: &String, token: &String) -> String {
        let s: NewSurveyData = serde_json::from_str(data).unwrap();

        let new_survey = Survey::new(s.into()).unwrap();

        // TODO: With ? the compiler fails because the internal repo Error that is
        // defined by the implementor does not have an explicit enough lifetime.
        self.repo.insert(&new_survey).unwrap();

        // TODO: Replace with actual json once we build output types.
        "SurveyOutputJsonHere".to_string()
}
}
