use domain_patterns::collections::Repository;
use crate::domain::survey::Survey;
use crate::application::commands::{CreateSurveyCommand, UpdateSurveyCommand};
use std::error::Error;
use std::convert::Into;
use crate::application::outputs::survey_data::SurveyOut;
use std::env::VarError::NotPresent;
use crate::application::services::decode_payload;
use std::thread::AccessError;

pub struct SurveyService<T> where
    T: Repository<Survey>
{
    repo: T,
}

impl<T> SurveyService<T> where
    T: Repository<Survey>
{
    fn new(repo: T) -> SurveyService<T> {
        SurveyService {
            repo,
        }
    }
}

impl<T: Repository<Survey>> SurveyService<T> {
    pub fn get_survey(&mut self, key: &String) -> Result<SurveyOut, Box<dyn Error>> {
        Ok(self.repo.get(key)?.ok_or(NotPresent)?.into())
    }

    pub fn create_survey(&mut self, command: CreateSurveyCommand) -> Result<SurveyOut, Box<dyn Error>> {
        let new_survey = Survey::new(command.into())?;

        self.repo.insert(&new_survey)?;

        Ok(
            new_survey.into()
        )
    }

    pub fn update_survey(&mut self, command: UpdateSurveyCommand) -> Result<SurveyOut, Box<dyn Error>> {
        let mut survey = self.repo.get(&command.id)?.ok_or(NotPresent)?;
        let requesting_author = decode_payload(&command.token).username;

        if !survey.belongs_to(requesting_author) {
            // Return useful in-house error here for translation to Not Authorized
        }

        survey.try_update(command.into())?;

        Ok(survey.into())
    }

    pub fn remove_survey(&mut self, key: &String) -> Result<(), Box<dyn Error>> {
        self.repo.remove(key)?;
        Ok(())
    }
}
