use domain_patterns::collections::Repository;
use survey_manager_core::survey::Survey;
use survey_manager_infra::mysql_repos::MysqlSurveyWriteRepository;
use survey_manager_infra::query_handlers::MysqlSurveyQueriesHandler;
use survey_manager_core::app_services::commands::SurveyCommandsHandler;

pub fn command_handler(conn: mysql::PooledConn) -> SurveyCommandsHandler<MysqlSurveyWriteRepository> {
    let repo = MysqlSurveyWriteRepository::new(conn);
    SurveyCommandsHandler::new(repo)
}

pub fn query_handler(conn: mysql::PooledConn) -> MysqlSurveyQueriesHandler {
    MysqlSurveyQueriesHandler::new(conn)
}
