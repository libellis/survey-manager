use survey_manager_infra::mysql_repos::{MysqlSurveyWriteRepository, MysqlSurveyDTOsRepository};
use survey_manager_core::app_services::commands::SurveyCommandsHandler;
use survey_manager_core::app_services::queries::SurveyQueriesHandler;
use survey_manager_infra::cache_repo_decorators::{RedisSurveyReadCacheRepository, RedisSurveyWriteCacheRepository};

pub fn command_handler() -> SurveyCommandsHandler<RedisSurveyWriteCacheRepository<MysqlSurveyWriteRepository>> {
    let primary_repo = MysqlSurveyWriteRepository::new();
    let cache_writing_repo = RedisSurveyWriteCacheRepository::new(primary_repo);
    SurveyCommandsHandler::new(cache_writing_repo)
}

/// Produces a query handler that is built with a wrapped repo, that's wrapped for caching abilities.
pub fn query_handler() -> SurveyQueriesHandler<RedisSurveyReadCacheRepository<MysqlSurveyDTOsRepository>> {
    let primary_repo = MysqlSurveyDTOsRepository::new();
    let cache_enhanced_repo = RedisSurveyReadCacheRepository::new(primary_repo);
    SurveyQueriesHandler::new(cache_enhanced_repo)
}
