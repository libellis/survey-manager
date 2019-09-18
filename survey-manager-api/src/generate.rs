use domain_patterns::collections::Repository;
use survey_manager_core::survey::Survey;
use survey_manager_infra::mysql_repos::{MysqlSurveyWriteRepository, MysqlSurveyDTOsRepository};
use survey_manager_core::app_services::commands::SurveyCommandsHandler;
use survey_manager_core::app_services::queries::SurveyQueriesHandler;

pub fn command_handler(conn: mysql::PooledConn) -> SurveyCommandsHandler<MysqlSurveyWriteRepository> {
    let repo = MysqlSurveyWriteRepository::new(conn);
    SurveyCommandsHandler::new(repo)
}

pub fn query_handler(conn: mysql::PooledConn) -> SurveyQueriesHandler<MysqlSurveyDTOsRepository> {
    let repo = MysqlSurveyDTOsRepository::new(conn);
    SurveyQueriesHandler::new(repo)
}
