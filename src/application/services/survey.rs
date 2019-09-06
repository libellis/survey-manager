use domain_patterns::collections::Repository;
use crate::domain::survey::Survey;
use crate::application::ports::inputs::NewSurveyData;
use std::error::Error;
use std::convert::Into;
use crate::application::ports::outputs::survey_data::SurveyOut;

pub struct SurveyService<T> where
    T: Repository<Survey>
{
    repo: T,
}

impl<T: Repository<Survey>> SurveyService<T> {

    // Creates a survey by taking in data which is a json string, a token
    // and finally returns a json string as output.
    pub fn create_survey(&mut self, data: &String) -> Result<String, Box<dyn Error>> {
        let s: NewSurveyData = serde_json::from_str(data)?;

        let new_survey = Survey::new(s.into())?;

        self.repo.insert(&new_survey)?;

        let response: SurveyOut = new_survey.into();

        Ok(
            serde_json::to_string(&response)?
        )
    }
}
