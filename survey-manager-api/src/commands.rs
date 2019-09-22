use actix_web::{web, Error as AWError};
use futures::Future;
use survey_manager_core::app_services::commands::SurveyCommands;
use domain_patterns::command::Handles;
use crate::generate;
use crate::error::CoreError;
use actix_web::error::BlockingError;

pub fn handle_command_async(
    cmd: SurveyCommands,
) -> impl Future<Item = String, Error = AWError> {
    web::block(move || generate::command_handler().handle(cmd) )
        .map_err(|e| {
            match e {
                BlockingError::Error(e) => CoreError(e),
                _ => CoreError::thread_fail(),
            }
        })
        .from_err()
}
