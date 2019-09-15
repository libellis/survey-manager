use domain_patterns::collections::Repository;
use survey_manager_core::survey::Survey;
use survey_manager_infra::mysql_repos::MysqlSurveyRepository;
use survey_manager_core::app_services::commands::SurveyCommandsHandler;

pub fn get_handler(conn: mysql::PooledConn) -> SurveyCommandsHandler<MysqlSurveyRepository> {
    let repo = MysqlSurveyRepository::new(conn);
    SurveyCommandsHandler::new(repo)
}