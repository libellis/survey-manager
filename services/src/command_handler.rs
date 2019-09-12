use domain_patterns::collections::Repository;
use domain_patterns::command::Handles;
use survey_manager_commands::*;
use survey_manager_domain::survey::Survey;
use super::{Result, Error, ErrorKind};

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
