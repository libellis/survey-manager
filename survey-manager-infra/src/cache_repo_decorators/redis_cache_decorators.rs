use survey_manager_core::app_services::repository_contracts::SurveyDTOReadRepository;
use crate::utils::redis_pool::{Conn, Pool, create_pool};
use survey_manager_core::dtos::{SurveyDTO, SurveyDTOs};

lazy_static! {
    static ref REDIS_POOL: Pool = {
        let cache_url = std::env::var("CACHE_URL").expect("CACHE_URL must be set");
        create_pool(&cache_url)
    };
}

pub struct RedisCacheRepository<T>
    where T: SurveyDTOReadRepository
{
    // A single connection to Mysql.  Handed down from a pool likely.
    cache: Conn,
    repo: T,
}

impl<T> RedisCacheRepository<T>
    where T: SurveyDTOReadRepository
{
    pub fn new(repo: T) -> RedisCacheRepository<T> {
        let pool = REDIS_POOL.clone();
        RedisCacheRepository {
            cache: pool.get().unwrap(),
            repo,
        }
    }
}

// TODO: We should consider caching on writes rather than setting up cache invalidation for this bounded context.
// Surveys are only accessed by their authors so any update on the write side should be what updates the cache.
// If the cache doesn't exist at all (wiped out, likely because it's on RAM), then we should refresh it as we're
// doing here.
impl<T> SurveyDTOReadRepository for RedisCacheRepository<T>
    where T: SurveyDTOReadRepository
{
    type Error = T::Error;

    fn get_survey_for_author(&mut self, id: &String, author: &String) -> Result<Option<SurveyDTO>, Self::Error> {
        let survey_result: Option<SurveyDTO> =
        // First we try to get it from redis
        if let Ok(survey_str) = redis::cmd("GET").arg(id).query::<String>(&mut *self.cache) {
            // If we succeed we deserialize the json string into the `SurveyDTO` object.
            Some(serde_json::from_str(&survey_str).unwrap())
        } else {
            // We didn't succeed so we attempt to grab it from the underlying repo we are wrapping (source of truth)
            let s_result: Option<SurveyDTO> = self.repo.get_survey_for_author(id, author)?;

            // If that was successful then we take the successful SurveyDTO, turn it into a string (json) and shove it into redis
            // so our cache is in sync.
            if let Some(survey) = &s_result {
                redis::cmd("SET").arg(id).arg(serde_json::to_string(survey).unwrap()).execute(&mut *self.cache);
            };

            // Regardless of success from underlying repo we return the result that came from the underlying storage (which could be None).
            s_result
        };

        // Here we simply wrap it in an Ok so it becomes a Result type
        Ok(survey_result)
    }

    fn get_surveys_by_author(&mut self, author: &String, lower_bound: usize, upper_bound: usize) -> Result<Option<SurveyDTOs>, Self::Error> {
        let survey_results: Option<SurveyDTOs> =
            // First we try to get it from redis
            if let Ok(surveys_str) = redis::cmd("GET").arg(author).query::<String>(&mut *self.cache) {
                // If we succeed we deserialize the json string into the `SurveyDTOs` object.
                Some(serde_json::from_str(&surveys_str).unwrap())
            } else {
                // We didn't succeed so we attempt to grab it from the underlying repo we are wrapping (source of truth)
                let s_results: Option<SurveyDTOs> = self.repo.get_surveys_by_author(author, lower_bound, upper_bound)?;

                // If that was successful then we take the successful SurveyDTOs, turn them into a string (json) and shove them into redis
                // so our cache is in sync.
                // TODO: Set key to be a combination of author and bound arguments.
                if let Some(surveys) = &s_results {
                    redis::cmd("SET").arg(author).arg(serde_json::to_string(surveys).unwrap()).execute(&mut *self.cache);
                };

                // Regardless of success from underlying repo we return the result that came from the underlying storage (which could be None).
                s_results
            };

        // Here we simply wrap it in an Ok so it becomes a Result type
        Ok(survey_results)
    }
}
