use domain_patterns::collections::Repository;
use crate::errors::Error::{ResourceNotFound, NotAuthorized, RepoFailure, ConcurrencyFailure};
use crate::errors::Result;
use domain_patterns::command::Handles;
use crate::survey::Survey;
use crate::app_services::commands::{CreateSurveyCommand, UpdateSurveyCommand, SurveyCommands, RemoveSurveyCommand};


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
    type Result = Result<String>;

    fn handle(&mut self, msg: CreateSurveyCommand) -> Result<String> {
        let new_survey = Survey::new(&msg)?;

        let s_id = self.repo.insert(&new_survey)
            .map_err(|e| RepoFailure { source: Box::new(e) })?;

        // Safe to unwrap.  If we had a duplicate key error, that's a database error and would
        // be returned above in the map err
        Ok(s_id.unwrap())
    }
}

impl<T: Repository<Survey>> Handles<UpdateSurveyCommand> for SurveyCommandsHandler<T> {
    type Result = Result<String>;

    fn handle(&mut self, msg: UpdateSurveyCommand) -> Result<String> {
        let mut survey = self.repo.get(&msg.id)
            .map_err(|e| RepoFailure { source: Box::new(e) })?
            .ok_or(ResourceNotFound { resource: format!("survey with id {}", &msg.id) })?;

        if !survey.belongs_to(&msg.author) {
            return Err(NotAuthorized.into());
        }

        survey.try_update(msg)?;

        let s_id = self.repo.update(&survey)
            .map_err(|e| RepoFailure { source: Box::new(e) })?;

        if let Some(s) = s_id {
            return Ok(s);
        }

        // If we got here then repo.update returned None.  This would only happen if there was no valid
        // survey to update, which could only have happened if the survey was deleted between the time
        // that we retrieved it with repo.get, and updated it with repo.update. Any other database errors
        // would have been returned inside the mapped RepoFailure error on the update.
        Err(ConcurrencyFailure)
    }
}

impl<T> Handles<RemoveSurveyCommand> for SurveyCommandsHandler<T>
    where T: Repository<Survey>
{
    type Result = Result<String>;

    fn handle(&mut self, msg: RemoveSurveyCommand) -> Self::Result {
        let survey = self.repo.get(&msg.id)
            .map_err(|e| RepoFailure { source: Box::new(e) })?
            .ok_or(ResourceNotFound { resource: format!("survey with id {}", &msg.id) })?;

        if !survey.belongs_to(&msg.requesting_author) {
            return Err(NotAuthorized.into());
        }

        let s_id = self.repo.remove(&msg.id)
            .map_err(|e| RepoFailure { source: Box::new(e) })?;

        Ok(s_id.unwrap())
    }
}

impl<T: Repository<Survey>> Handles<SurveyCommands> for SurveyCommandsHandler<T> {
    type Result = Result<String>;

    fn handle(&mut self, msg: SurveyCommands) -> Result<String> {
        match msg {
            SurveyCommands::CreateSurveyCommand(cmd) => self.handle(cmd),
            SurveyCommands::UpdateSurveyCommand(cmd) => self.handle(cmd),
            SurveyCommands::RemoveSurveyCommand(cmd) => self.handle(cmd),
        }
    }
}
