use actix_web::web;
use futures::Future;
use survey_manager_core::app_services::commands::SurveyCommands;
use domain_patterns::command::Handles;
use crate::generate;
use crate::error::ApiError;

pub fn handle_command_async(
    cmd: SurveyCommands,
) -> impl Future<Item = String, Error = ApiError> {
    web::block(move || generate::command_handler().handle(cmd) )
        .from_err()
}
