use domain_patterns::query::HandlesQuery;
use survey_manager_core::app_services::queries::{FindSurveyQuery, FindSurveysByAuthorQuery, SurveyQueries, PageConfig};
use survey_manager_core::dtos::{SurveyDTO, SurveyDTOs, ListViewSurveyDTO};
use survey_manager_core::app_services::repository_contracts::SurveyDTOReadRepository;

pub struct MysqlSurveyDTOsRepository {
    // A single connection to Mysql.  Handed down from a pool likely.
    conn: mysql::PooledConn,
}

impl MysqlSurveyDTOsRepository {
    pub fn new() -> MysqlSurveyDTOsRepository {
        let mut pool = super::MYSQL_POOL.clone();
        MysqlSurveyDTOsRepository {
            conn: pool.get_conn().unwrap()
        }
    }
}

impl SurveyDTOReadRepository for MysqlSurveyDTOsRepository {
    type Error = mysql::Error;

    fn get_survey_for_author(&mut self, id: &String, author: &String) -> Result<Option<SurveyDTO>, mysql::Error> {
        let survey_result: Option<SurveyDTO> =
            match self.conn.prep_exec(
                "SELECT survey_data FROM survey WHERE id=? AND author=?",
                (id, author)
            ) {
                Ok(mut q_result) => {
                    if let Some(row_result)  = q_result.next() {
                        let row = row_result?;
                        let survey_data: String = mysql::from_row(row);
                        Some(serde_json::from_str(&survey_data).unwrap())
                    } else {
                        None
                    }
                },
                Err(e) => {
                    return Err(e);
                },
            };

        Ok(survey_result)
    }

    fn get_surveys_by_author(&mut self, author: &String, lower_bound: usize, upper_bound: usize) -> Result<Option<SurveyDTOs>, mysql::Error> {
        let survey_results: Option<SurveyDTOs> =
            match self.conn.prep_exec(
                "SELECT id, author, title, category FROM survey WHERE author=? LIMIT ?,?",
                (author, lower_bound, upper_bound)
            ) {
                Ok(mut q_result) => {
                    let mut surveys = Vec::new();
                    for row_result in q_result {
                        let row = row_result?;
                        let (id, author, title, category) = mysql::from_row(row);
                        let s_dto = ListViewSurveyDTO {
                            id,
                            author,
                            title,
                            category,
                        };
                        surveys.push(s_dto);
                    }
                    if surveys.len() == 0 {
                        None
                    } else {
                        Some(
                            SurveyDTOs {
                                surveys,
                            }
                        )
                    }
                },
                Err(e) => {
                    return Err(e);
                },
            };

        Ok(survey_results)
    }
}
