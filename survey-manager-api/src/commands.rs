use actix_web::{web, Error as AWError};
use futures::{Future, IntoFuture};
use mysql;
use survey_manager_core::app_services::commands::{SurveyCommands, CreateSurveyCommand};
use domain_patterns::command::Handles;
use survey_manager_core::survey::Survey;
use crate::generate::command_handler;
use crate::error::Error;
use mysql::PooledConn;

pub type Pool = mysql::Pool;
pub type Conn = mysql::PooledConn;

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
