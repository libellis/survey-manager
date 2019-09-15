use domain_patterns::command::Command;
use domain_patterns::message::Message;
use crate::app_services::commands::{CreateSurveyCommand, UpdateSurveyCommand};
use domain_patterns::collections::Repository;
use crate::survey::Survey;
use std::convert::Into;
use crate::Error;
use crate::errors::Error::{ResourceNotFound, NotAuthorized, RepoFailure};
use crate::errors::Result;
use domain_patterns::command::Handles;
use snafu::ResultExt;

#[derive(Clone, Command)]
pub enum SurveyCommands {
    CreateSurveyCommand(CreateSurveyCommand),
    UpdateSurveyCommand(UpdateSurveyCommand),
}

// Implementations to automatically turn each variant into the parent enum.
impl From<CreateSurveyCommand> for SurveyCommands {
    fn from(cmd: CreateSurveyCommand) -> Self {
        SurveyCommands::CreateSurveyCommand(cmd)
    }
}

impl From<UpdateSurveyCommand> for SurveyCommands {
    fn from(cmd: UpdateSurveyCommand) -> Self {
        SurveyCommands::UpdateSurveyCommand(cmd)
    }
}


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
    type Result = Result<Option<String>>;

    fn handle(&mut self, msg: CreateSurveyCommand) -> Result<Option<String>> {
        let new_survey = Survey::new(&msg)?;

        let s_id = self.repo.insert(&new_survey)
            .map_err(|e| RepoFailure { source: Box::new(e) })?;

        Ok(s_id)
    }
}

impl<T: Repository<Survey>> Handles<UpdateSurveyCommand> for SurveyCommandsHandler<T> {
    type Result = Result<Option<String>>;

    fn handle(&mut self, msg: UpdateSurveyCommand) -> Result<Option<String>> {
        let mut survey = self.repo.get(&msg.id)
            .map_err(|e| RepoFailure { source: Box::new(e) })?
            .ok_or(ResourceNotFound { resource: format!("survey with id {}", &msg.id) })?;

        if !survey.belongs_to(&msg.author) {
            return Err(NotAuthorized.into());
        }

        survey.try_update(msg)?;

        let s_id = self.repo.update(&survey)
            .map_err(|e| RepoFailure { source: Box::new(e) })?;

        Ok(s_id)
    }
}

impl<T: Repository<Survey>> Handles<SurveyCommands> for SurveyCommandsHandler<T> {
    type Result = Result<Option<String>>;

    fn handle(&mut self, msg: SurveyCommands) -> Result<Option<String>> {
        match msg {
            SurveyCommands::CreateSurveyCommand(cmd) => self.handle(cmd),
            SurveyCommands::UpdateSurveyCommand(cmd) => self.handle(cmd),
        }
    }
}
