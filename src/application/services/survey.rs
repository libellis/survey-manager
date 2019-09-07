use domain_patterns::collections::Repository;
use crate::domain::survey::Survey;
use crate::application::inputs::NewSurveyData;
use std::error::Error;
use std::convert::Into;
use crate::application::outputs::survey_data::SurveyOut;

pub struct SurveyService<T> where
    T: Repository<Survey>
{
    repo: T,
}

impl<T: Repository<Survey>> SurveyService<T> {

    // Creates a survey by taking in data which is a json string, a token
    // and finally returns a json string as output.
    pub fn create_survey(&mut self, survey_data: NewSurveyData) -> Result<SurveyOut, Box<dyn Error>> {
        let new_survey = Survey::new(survey_data.into())?;

        self.repo.insert(&new_survey)?;

        Ok(
            new_survey.into()
        )
    }
}
