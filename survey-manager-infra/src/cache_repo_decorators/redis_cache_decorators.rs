use survey_manager_core::app_services::repository_contracts::SurveyDTOReadRepository;
use crate::utils::redis_pool::{Conn, Pool, create_pool};
use survey_manager_core::dtos::{SurveyDTO, SurveyDTOs};
use domain_patterns::collections::Repository;
use survey_manager_core::survey::Survey;

lazy_static! {
    static ref REDIS_POOL: Pool = {
        let cache_url = std::env::var("CACHE_URL").expect("CACHE_URL must be set");
        create_pool(&cache_url)
    };
}

pub struct RedisSurveyReadCacheRepository<T>
    where T: SurveyDTOReadRepository
{
    // A single connection to Mysql.  Handed down from a pool likely.
    cache: Conn,
    repo: T,
}

impl<T> RedisSurveyReadCacheRepository<T>
    where T: SurveyDTOReadRepository
{
    pub fn new(repo: T) -> RedisSurveyReadCacheRepository<T> {
        RedisSurveyReadCacheRepository {
            cache: REDIS_POOL.clone().get().unwrap(),
            repo,
        }
    }
}

impl<T> SurveyDTOReadRepository for RedisSurveyReadCacheRepository<T>
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

    fn get_surveys_by_author(&mut self, author: &String) -> Result<Option<SurveyDTOs>, Self::Error> {
        let key = format!("{}_surveys", author);
        let survey_results: Option<SurveyDTOs> =
            // First we try to get it from redis
            if let Ok(surveys_str) = redis::cmd("GET").arg(&key).query::<String>(&mut *self.cache) {
                // If we succeed we deserialize the json string into the `SurveyDTOs` object.
                Some(serde_json::from_str(&surveys_str).unwrap())
            } else {
                // We didn't succeed so we attempt to grab it from the underlying repo we are wrapping (source of truth).
                let s_results: Option<SurveyDTOs> = self.repo.get_surveys_by_author(author)?;

                // If that was successful then we take the successful SurveyDTOs, turn them into a string (json) and shove them into redis
                // so our cache is in sync.
                if let Some(surveys) = &s_results {
                    redis::cmd("SET").arg(&key).arg(serde_json::to_string(surveys).unwrap()).execute(&mut *self.cache);
                };

                s_results
            };

        // Here we simply wrap it in an Ok so it becomes a Result type
        Ok(survey_results)
    }
}

// This wrapper is intended to write to the cache on writes, and otherwise is a pass through on all gets.
// Gets are for write side of model, so we should always pass through to the real database on those gets.
pub struct RedisSurveyWriteCacheRepository<T>
    where T: Repository<Survey>
{
    // A single connection to Mysql.  Handed down from a pool likely.
    cache: Conn,
    repo: T,
}

impl<T> RedisSurveyWriteCacheRepository<T>
    where T: Repository<Survey>
{
    pub fn new(repo: T) -> RedisSurveyWriteCacheRepository<T> {
        RedisSurveyWriteCacheRepository {
            cache: REDIS_POOL.clone().get().unwrap(),
            repo,
        }
    }

    // Resets cache of all surveys by author_surveys
    pub fn invalidate_surveys_cache(&mut self, author: String) {
        let key = format!("{}_surveys", author);
        redis::cmd("DEL")
            .arg(&key)
            .execute(&mut *self.cache);
    }
}

impl<T> Repository<Survey> for RedisSurveyWriteCacheRepository<T>
    where T: Repository<Survey>
{
    type Error = T::Error;

    // Insert into underlying persistent storage, then set the survey into redis cache.
    // Lastly invalidate the cache for aggregate surveys (all surveys).  That gets refreshed
    // On read rather than on write.
    fn insert(&mut self, entity: &Survey) -> Result<Option<String>, Self::Error> {
        let maybe_id = self.repo.insert(entity)?;
        redis::cmd("SET")
            .arg(entity.author().to_string())
            .arg(serde_json::to_string(&SurveyDTO::from(entity)).unwrap())
            .execute(&mut *self.cache);
        self.invalidate_surveys_cache(entity.author().to_string());
        Ok(maybe_id)
    }

    // passthrough
    fn get(&mut self, key: &String) -> Result<Option<Survey>, Self::Error> {
        self.repo.get(key)
    }

    // passthrough
    fn get_paged(&mut self, page_num: usize, page_size: usize) -> Result<Option<Vec<Survey>>, Self::Error> {
        self.repo.get_paged(page_num, page_size)
    }

    // Update in underlying persistent storage, then update the survey in redis cache.
    // Lastly invalidate the cache for aggregate surveys (all surveys).  That gets refreshed
    // On read rather than on write.
    fn update(&mut self, entity: &Survey) -> Result<Option<String>, Self::Error> {
        let maybe_id = self.repo.update(entity)?;
        redis::cmd("SET")
            .arg(entity.author().to_string())
            .arg(serde_json::to_string(&SurveyDTO::from(entity)).unwrap())
            .execute(&mut *self.cache);
        self.invalidate_surveys_cache(entity.author().to_string());
        Ok(maybe_id)
    }

    // Remove from underlying storage and remove survey from redis cache.
    fn remove(&mut self, key: &String) -> Result<Option<String>, Self::Error> {
        let s_id = self.repo.get(key)?;
        // Invalidate caches before delete.
        if let Some(s) = s_id {
            self.invalidate_surveys_cache(s.author().to_string());
            redis::cmd("DEL")
                .arg(key)
                .execute(&mut *self.cache);
        } else {
            // No survey so we can just leave here without trying to delete.
            return Ok(None);
        }

        let maybe_id = self.repo.remove(key)?;
        Ok(maybe_id)
    }
}