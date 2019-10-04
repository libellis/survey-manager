#[macro_use]
extern crate criterion;

use criterion::Criterion;
use dotenv::dotenv;
use survey_manager_infra::mysql_repos::MysqlSurveyDTOsRepository;
use survey_manager_core::app_services::repository_contracts::SurveyDTOReadRepository;
use survey_manager_infra::cache_repo_decorators::RedisSurveyReadCacheRepository;

fn benchmark_reading_no_cache(c: &mut Criterion) {
    dotenv().ok();
    let author = "test_user".to_string();
    let s_id = "9324f63d-545b-47fb-be7d-f560bb7476ef".to_string();
    let mut mysql_repo = MysqlSurveyDTOsRepository::new();
    c.bench_function("Mysql repo reading with no cache layer.", |b| {
        b.iter(|| {
            mysql_repo.get_survey_for_author(&s_id, &author)
        });
    });
}

fn benchmark_reading_redis_cache(c: &mut Criterion) {
    dotenv().ok();
    let mut mysql_repo = MysqlSurveyDTOsRepository::new();
    let mut cached_repo = RedisSurveyReadCacheRepository::new(mysql_repo);
    let author = "test_user".to_string();
    let s_id = "9324f63d-545b-47fb-be7d-f560bb7476ef".to_string();
    c.bench_function("Mysql repo reading with redis cache layer.", |b| {
        b.iter(|| {
            cached_repo.get_survey_for_author(&s_id, &author);
        });
    });
}

criterion_group!(benches, benchmark_reading_no_cache, benchmark_reading_redis_cache);

criterion_main!(benches);
