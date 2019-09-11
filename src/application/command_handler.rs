use domain_patterns::collections::Repository;
use crate::domain::survey::Survey;
use crate::application::inputs::{CreateSurveyDTO, UpdateSurveyDTO};
use std::convert::Into;
use crate::Error;
use crate::application::outputs::survey_data::SurveyOut;
use crate::application::services::decode_payload;
use crate::errors::ErrorKind::{ResourceNotFound, NotAuthorized};
use crate::errors::Result;
use domain_patterns::command::Handles;
use crate::domain::survey::commands::{SurveyCommands, CreateSurveyCommand};

/// TODO: Figure out how to coalesce the error return types since we don't know
/// the error type of SurveyService.
pub struct SurveyCommandsHandler<T> where
    T: Repository<Survey>
{
    repo: T,
}

impl<T> SurveyCommandsHandler<T> where
    T: Repository<Survey>
{
    pub fn new(repo: T) -> SurveyCommandsHandler<T> {
        SurveyCommandsHandler {
            repo,
        }
    }
}

impl<T: Repository<Survey>> Handles<CreateSurveyCommand> for SurveyCommandsHandler<T> {
    type Error = Error;

    fn handle(&mut self, msg: &CreateSurveyCommand) -> Result<()> {
        let new_survey = Survey::new(msg)?;

        self.repo.insert(&new_survey)?;

        Ok(())
    }
}

impl<T: Repository<Survey>> Handles<SurveyCommands> for SurveyCommandsHandler<T> {
    type Error = Error;

    fn handle(&mut self, msg: &SurveyCommands) -> Result<()> {
        match msg {
            SurveyCommands::CreateSurveyCommand(cmd) => self.handle(cmd)
        }
    }
}
