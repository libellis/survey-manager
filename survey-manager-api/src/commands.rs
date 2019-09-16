use actix_web::{web, Error as AWError};
use futures::{Future, IntoFuture};
use mysql;
use survey_manager_core::app_services::commands::{SurveyCommands, CreateSurveyCommand};
use domain_patterns::command::Handles;
use survey_manager_core::survey::Survey;
use crate::generate::command_handler;
use survey_manager_core::Error;
use mysql::PooledConn;

pub type Pool = mysql::Pool;
pub type Conn = mysql::PooledConn;

pub fn handle_command_async(
    pool: &Pool,
    cmd: SurveyCommands,
) -> impl Future<Item = Option<String>, Error = AWError> {
    let pool = pool.clone();
    web::block(move || handle(pool.get_conn().unwrap(), cmd))
        .from_err()
}

pub fn handle(
    conn: PooledConn,
    cmd: SurveyCommands,
) -> Result<Option<String>, String> {
    command_handler(conn).handle(cmd).map_err(|e| format!("{}", e))
}
