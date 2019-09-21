use actix_web::{web, Error as AWError};
use futures::Future;
use survey_manager_core::app_services::commands::SurveyCommands;
use domain_patterns::command::Handles;
use crate::generate::command_handler;
use crate::error::Error;

pub fn handle_command_async(
    cmd: SurveyCommands,
) -> impl Future<Item = Option<String>, Error = AWError> {
    web::block(move || handle(cmd))
        .from_err()
}

pub fn handle(
    cmd: SurveyCommands,
) -> Result<Option<String>, Error> {
    command_handler()
        .handle(cmd)
        .map_err(|e| Error::from(e))
}
