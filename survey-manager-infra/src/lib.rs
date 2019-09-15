pub mod mysql_repos;
pub mod errors;
pub use errors::*;
pub mod utils;

#[cfg(test)]
mod tests {
    use crate::mysql_repos::MysqlSurveyRepository;
    use domain_patterns::collections::Repository;
    use domain_patterns::models::Entity;
    use survey_manager_core::survey::Survey;
    use survey_manager_core::app_services::commands::{CreateSurveyCommand, CreateQuestionCommand, CreateChoiceCommand};
    use dotenv::dotenv;
    use std::env;

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
}
