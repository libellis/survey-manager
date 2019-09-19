use domain_patterns::collections::Repository;
use survey_manager_core::survey::Survey;
use survey_manager_infra::mysql_repos::{MysqlSurveyWriteRepository, MysqlSurveyDTOsRepository};
use survey_manager_core::app_services::commands::SurveyCommandsHandler;
use survey_manager_core::app_services::queries::SurveyQueriesHandler;
use survey_manager_infra::cache_repo_decorators::RedisCacheRepository;

pub fn command_handler() -> SurveyCommandsHandler<MysqlSurveyWriteRepository> {
    let repo = MysqlSurveyWriteRepository::new();
    SurveyCommandsHandler::new(repo)
}

/// Produces a query handler that is built with a wrapped repo, that's wrapped for caching abilities.
pub fn query_handler() -> SurveyQueriesHandler<RedisCacheRepository<MysqlSurveyDTOsRepository>> {
    let primary_repo = MysqlSurveyDTOsRepository::new();
    let cache_enhanced_repo = RedisCacheRepository::new(primary_repo);
    SurveyQueriesHandler::new(cache_enhanced_repo)
}

pub fn query_handler_no_cache() -> SurveyQueriesHandler<MysqlSurveyDTOsRepository> {
    let primary_repo = MysqlSurveyDTOsRepository::new();
    SurveyQueriesHandler::new(primary_repo)
}
