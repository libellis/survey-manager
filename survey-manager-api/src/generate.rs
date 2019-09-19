use domain_patterns::collections::Repository;
use survey_manager_core::survey::Survey;
use survey_manager_infra::mysql_repos::{MysqlSurveyWriteRepository, MysqlSurveyDTOsRepository};
use survey_manager_infra::utils::redis_pool::Conn as RedisConn;
use survey_manager_core::app_services::commands::SurveyCommandsHandler;
use survey_manager_core::app_services::queries::SurveyQueriesHandler;
use survey_manager_infra::cache_repo_decorators::RedisCacheRepository;
use r2d2_redis::RedisConnectionManager;
use survey_manager_infra::utils::redis_pool::create_pool;

lazy_static! {
    static ref MYSQL_POOL: mysql::Pool = {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        mysql::Pool::new(&database_url).unwrap()
    };
}

lazy_static! {
    static ref REDIS_POOL: r2d2::Pool<RedisConnectionManager> = {
        let cache_url = std::env::var("CACHE_URL").expect("CACHE_URL must be set");
        create_pool(&cache_url)
    };
}

pub fn command_handler() -> SurveyCommandsHandler<MysqlSurveyWriteRepository> {
    let mysql_pool = MYSQL_POOL.clone();
    let repo = MysqlSurveyWriteRepository::new(mysql_pool.get_conn().unwrap());
    SurveyCommandsHandler::new(repo)
}

/// Produces a query handler that is built with a wrapped repo, that's wrapped for caching abilities.
pub fn query_handler() -> SurveyQueriesHandler<RedisCacheRepository<MysqlSurveyDTOsRepository>> {
    let mysql_pool = MYSQL_POOL.clone();
    let redis_pool = REDIS_POOL.clone();
    let primary_repo = MysqlSurveyDTOsRepository::new(mysql_pool.get_conn().unwrap());
    let cache_enhanced_repo = RedisCacheRepository::new(redis_pool.get().unwrap(), primary_repo);
    SurveyQueriesHandler::new(cache_enhanced_repo)
}

pub fn query_handler_no_cache() -> SurveyQueriesHandler<MysqlSurveyDTOsRepository> {
    let pool = MYSQL_POOL.clone();
    let primary_repo = MysqlSurveyDTOsRepository::new(pool.get_conn().unwrap());
    SurveyQueriesHandler::new(primary_repo)
}
