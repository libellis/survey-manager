use domain_patterns::command::Command;
use domain_patterns::message::Message;
use crate::app_services::commands::{CreateSurveyCommand, UpdateSurveyCommand};

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
