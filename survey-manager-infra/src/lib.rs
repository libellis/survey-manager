pub mod mysql_repos;
pub mod errors;
pub use errors::*;
pub mod utils;

#[cfg(test)]
mod tests {
    use crate::mysql_repos::MysqlSurveyRepository;
    use domain_patterns::collections::Repository;
    use domain_patterns::models::{Entity, AggregateRoot};
    use survey_manager_core::survey::Survey;
    use survey_manager_core::app_services::commands::{CreateSurveyCommand, CreateQuestionCommand, CreateChoiceCommand, UpdateSurveyCommand};
    use dotenv::dotenv;
    use std::env;
    use std::convert::TryFrom;
    use survey_manager_core::value_objects::Title;

    fn create_test_survey() -> Survey {
        let choice = CreateChoiceCommand {
            content: None,
            content_type: "youtube".to_string(),
            title: "test choice title".to_string()
        };

        let question = CreateQuestionCommand {
            question_type: "ranked".to_string(),
            title: "test question title".to_string(),
            choices: vec![choice]
        };

        let create_survey_command = CreateSurveyCommand {
            author: "test_author".to_string(),
            title: "test survey title".to_string(),
            description: "test survey description".to_string(),
            category: "music".to_string(),
            questions: vec![question]
        };

        Survey::new(&create_survey_command).unwrap()
    }

    #[test]
    fn it_works() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        let pool = mysql::Pool::new(&database_url).unwrap();
        {
            let mut conn = pool.get_conn().unwrap();
            conn.query(r"CREATE TEMPORARY TABLE survey (
                         id VARCHAR(64) PRIMARY KEY,
                         version BIGINT UNSIGNED NOT NULL,
                         author VARCHAR(64) NOT NULL,
                         title VARCHAR(128) NOT NULL,
                         category VARCHAR(64) NOT NULL,
                         created_on BIGINT NOT NULL,
                         survey_data JSON NOT NULL
                     )").unwrap();
            let mut survey_repo = MysqlSurveyRepository::new(conn);
            let survey = create_test_survey();
            survey_repo.insert(&survey).unwrap();

            let retrieved = survey_repo.get(&survey.id()).unwrap();
            assert!(retrieved.is_some());
            assert_eq!(&retrieved.unwrap().id(), &survey.id());
        }
    }

    #[test]
    fn duplicate_insert_returns_none() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        let pool = mysql::Pool::new(&database_url).unwrap();
        {
            let mut conn = pool.get_conn().unwrap();
            conn.query(r"CREATE TEMPORARY TABLE survey (
                         id VARCHAR(64) PRIMARY KEY,
                         version BIGINT UNSIGNED NOT NULL,
                         author VARCHAR(64) NOT NULL,
                         title VARCHAR(128) NOT NULL,
                         category VARCHAR(64) NOT NULL,
                         created_on BIGINT NOT NULL,
                         survey_data JSON NOT NULL
                     )").unwrap();
            let mut survey_repo = MysqlSurveyRepository::new(conn);
            let survey = create_test_survey();
            survey_repo.insert(&survey).unwrap();

            // This should return None.
            let none = survey_repo.insert(&survey).unwrap();

            assert!(none.is_none());
        }
    }

    #[test]
    fn survey_update_works() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        let pool = mysql::Pool::new(&database_url).unwrap();
        {
            let mut conn = pool.get_conn().unwrap();
            conn.query(r"CREATE TEMPORARY TABLE survey (
                         id VARCHAR(64) PRIMARY KEY,
                         version BIGINT UNSIGNED NOT NULL,
                         author VARCHAR(64) NOT NULL,
                         title VARCHAR(128) NOT NULL,
                         category VARCHAR(64) NOT NULL,
                         created_on BIGINT NOT NULL,
                         survey_data JSON NOT NULL
                     )").unwrap();
            let mut survey_repo = MysqlSurveyRepository::new(conn);
            let mut survey = create_test_survey();
            survey_repo.insert(&survey).unwrap();

            let new_title = Title::try_from("test_title".to_string()).unwrap();
            let survey_update_command = UpdateSurveyCommand {
                id: survey.id(),
                author: "".to_string(),
                title: Some(new_title.to_string()),
                description: None,
                category: None,
                questions: None
            };

            survey.try_update(&survey_update_command).unwrap();

            survey_repo.update(&survey).unwrap();

            let updated_survey = survey_repo.get(&survey.id()).unwrap();

            assert_eq!(&updated_survey.unwrap().title().to_string(), &new_title.to_string());
        }
    }

    #[test]
    fn survey_remove_works() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        let pool = mysql::Pool::new(&database_url).unwrap();
        {
            let mut conn = pool.get_conn().unwrap();
            conn.query(r"CREATE TEMPORARY TABLE survey (
                         id VARCHAR(64) PRIMARY KEY,
                         version BIGINT UNSIGNED NOT NULL,
                         author VARCHAR(64) NOT NULL,
                         title VARCHAR(128) NOT NULL,
                         category VARCHAR(64) NOT NULL,
                         created_on BIGINT NOT NULL,
                         survey_data JSON NOT NULL
                     )").unwrap();

            let mut survey_repo = MysqlSurveyRepository::new(conn);
            let survey = create_test_survey();
            survey_repo.insert(&survey).unwrap();

            // Make sure that worked before trying to delete.
            let retrieved = survey_repo.get(&survey.id()).unwrap();
            assert!(retrieved.is_some());
            assert_eq!(&retrieved.unwrap().id(), &survey.id());

            let d_id = survey_repo.remove(&survey.id()).unwrap();
            assert_eq!(&d_id.unwrap(), &survey.id());

            // Make sure we really did delete it.
            let retrieved = survey_repo.get(&survey.id()).unwrap();
            assert!(retrieved.is_none());
        }
    }

    #[test]
    fn invalid_pk_returns_none_for_get() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        let pool = mysql::Pool::new(&database_url).unwrap();
        {
            let mut conn = pool.get_conn().unwrap();
            conn.query(r"CREATE TEMPORARY TABLE survey (
                         id VARCHAR(64) PRIMARY KEY,
                         version BIGINT UNSIGNED NOT NULL,
                         author VARCHAR(64) NOT NULL,
                         title VARCHAR(128) NOT NULL,
                         category VARCHAR(64) NOT NULL,
                         created_on BIGINT NOT NULL,
                         survey_data JSON NOT NULL
                     )").unwrap();
            let mut survey_repo = MysqlSurveyRepository::new(conn);
            let mut survey = create_test_survey();
            survey_repo.insert(&survey).unwrap();

            let none = survey_repo.get(&"wrong_key".to_string()).unwrap();

            assert!(none.is_none());
        }
    }

    #[test]
    fn invalid_pk_returns_none_for_update() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        let pool = mysql::Pool::new(&database_url).unwrap();
        {
            let mut conn = pool.get_conn().unwrap();
            conn.query(r"CREATE TEMPORARY TABLE survey (
                         id VARCHAR(64) PRIMARY KEY,
                         version BIGINT UNSIGNED NOT NULL,
                         author VARCHAR(64) NOT NULL,
                         title VARCHAR(128) NOT NULL,
                         category VARCHAR(64) NOT NULL,
                         created_on BIGINT NOT NULL,
                         survey_data JSON NOT NULL
                     )").unwrap();

            let mut survey_repo = MysqlSurveyRepository::new(conn);
            let survey = create_test_survey();
            survey_repo.insert(&survey).unwrap();

            // Make sure that worked before trying to delete.
            let retrieved = survey_repo.get(&survey.id()).unwrap();
            assert!(retrieved.is_some());
            assert_eq!(&retrieved.unwrap().id(), &survey.id());

            survey_repo.remove(&survey.id()).unwrap();

            let none = survey_repo.update(&survey).unwrap();
            assert!(none.is_none());
        }
    }

    #[test]
    fn remove_twice_yields_none() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        let pool = mysql::Pool::new(&database_url).unwrap();
        {
            let mut conn = pool.get_conn().unwrap();
            conn.query(r"CREATE TEMPORARY TABLE survey (
                         id VARCHAR(64) PRIMARY KEY,
                         version BIGINT UNSIGNED NOT NULL,
                         author VARCHAR(64) NOT NULL,
                         title VARCHAR(128) NOT NULL,
                         category VARCHAR(64) NOT NULL,
                         created_on BIGINT NOT NULL,
                         survey_data JSON NOT NULL
                     )").unwrap();

            let mut survey_repo = MysqlSurveyRepository::new(conn);
            let survey = create_test_survey();
            survey_repo.insert(&survey).unwrap();

            // Make sure that worked before trying to delete.
            let retrieved = survey_repo.get(&survey.id()).unwrap();
            assert!(retrieved.is_some());
            assert_eq!(&retrieved.unwrap().id(), &survey.id());

            survey_repo.remove(&survey.id()).unwrap();
            let none = survey_repo.remove(&survey.id()).unwrap();

            assert!(none.is_none());
        }
    }
}
