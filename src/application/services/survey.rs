use domain_patterns::collections::Repository;
use crate::domain::survey::Survey;
use crate::application::commands::{CreateSurveyCommand, UpdateSurveyCommand};
use std::error::Error;
use std::convert::Into;
use crate::application::outputs::survey_data::SurveyOut;
use crate::application::services::decode_payload;
use crate::errors::ErrorKind::{ResourceNotFound, NotAuthorized};
use crate::errors::Result;

/// TODO: Figure out how to coalesce the error return types since we don't know
/// the error type of SurveyService.
pub struct SurveyService<T> where
    T: Repository<Survey>
{
    repo: T,
}

impl<T> SurveyService<T> where
    T: Repository<Survey>
{
    pub fn new(repo: T) -> SurveyService<T> {
        SurveyService {
            repo,
        }
    }
}

impl<T: Repository<Survey>> SurveyService<T> {
    pub fn get_survey(&mut self, key: &String) -> Result<SurveyOut> {
        Ok(
            self.repo.get(key)?
                .ok_or(ResourceNotFound(format!("survey with id {}", key)))?.into()
        )
    }

    pub fn create_survey(&mut self, command: CreateSurveyCommand) -> Result<SurveyOut> {
        let new_survey = Survey::new(command.into())?;

        self.repo.insert(&new_survey)?;

        Ok(
            new_survey.into()
        )
    }

    pub fn update_survey(&mut self, command: UpdateSurveyCommand) -> Result<SurveyOut> {
        let mut survey = self.repo.get(&command.id)?
            .ok_or(ResourceNotFound(format!("survey with id {}", &command.id)))?;

        let requesting_author = decode_payload(&command.token).username;

        if !survey.belongs_to(&requesting_author) {
            return Err(NotAuthorized.into())
        }

        survey.try_update(command.into())?;

        Ok(survey.into())
    }

    pub fn remove_survey(&mut self, key: &String) -> Result<()> {
        self.repo.remove(key)?;
        Ok(())
    }
}
