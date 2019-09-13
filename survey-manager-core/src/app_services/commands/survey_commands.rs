use domain_patterns::command::Command;
use domain_patterns::message::Message;
use crate::app_services::commands::{CreateSurveyCommand, UpdateSurveyCommand};
use domain_patterns::collections::Repository;
use crate::survey::Survey;
use std::convert::Into;
use crate::Error;
use crate::errors::ErrorKind::{ResourceNotFound, NotAuthorized, DbFailure};
use crate::errors::Result;
use domain_patterns::command::Handles;

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
    type Error = Error;

    fn handle(&mut self, msg: &CreateSurveyCommand) -> Result<()> {
        let new_survey = Survey::new(msg)?;

        if let Err(_) = self.repo.insert(&new_survey) {
            return Err(DbFailure.into());
        }

        Ok(())
    }
}

impl<T: Repository<Survey>> Handles<UpdateSurveyCommand> for SurveyCommandsHandler<T> {
    type Error = Error;

    fn handle(&mut self, msg: &UpdateSurveyCommand) -> Result<()> {
        // TODO: This is much cleaner, figure out how to make this work.
//        let mut survey = self.repo.get(&msg.id)
//            .map_err(|_e: <T as Repository<Survey>>::Error| DbFailure.into())?
//            .ok_or(ResourceNotFound(format!("survey with id {}", &msg.id)))?;

        let mut survey = if let Ok(s) = self.repo.get(&msg.id) {
            s.ok_or(ResourceNotFound(format!("survey with id {}", &msg.id)))?
        } else {
            return Err(DbFailure.into());
        };

        if !survey.belongs_to(&msg.author) {
            return Err(NotAuthorized.into());
        }

        survey.try_update(msg)?;

        Ok(())
    }
}

impl<T: Repository<Survey>> Handles<SurveyCommands> for SurveyCommandsHandler<T> {
    type Error = Error;

    fn handle(&mut self, msg: &SurveyCommands) -> Result<()> {
        match msg {
            SurveyCommands::CreateSurveyCommand(cmd) => self.handle(cmd),
            SurveyCommands::UpdateSurveyCommand(cmd) => self.handle(cmd),
        }
    }
}