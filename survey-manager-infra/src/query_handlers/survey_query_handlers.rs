use domain_patterns::query::HandlesQuery;
use survey_manager_core::app_services::queries::{FindSurveyQuery, FindSurveysByAuthorQuery, SurveyQueries, PageConfig};
use survey_manager_core::dtos::SurveyDTO;

// TODO: For now this talks to the same model as the write model.  Should probably change this to read from cache like redis.
pub struct MysqlSurveyQueriesHandler {
    // A single connection to Mysql.  Handed down from a pool likely.
    conn: mysql::PooledConn,
}

impl MysqlSurveyQueriesHandler {
    pub fn new(conn: mysql::PooledConn) -> MysqlSurveyQueriesHandler {
        MysqlSurveyQueriesHandler {
            conn,
        }
    }
}

impl HandlesQuery<FindSurveyQuery> for MysqlSurveyQueriesHandler {
    // String in this case is just the pure JSON.
    // no need to turn it into a data structure - we are just giving the caller
    // json anyways.
    type Result = Result<Option<String>, mysql::Error>;

    fn handle(&mut self, query: FindSurveyQuery) -> Self::Result {
        let survey_result: Option<String> =
            match self.conn.prep_exec(
                "SELECT survey_data FROM survey WHERE id=? AND author=?",
                (query.id, query.requesting_author)
            ) {
                Ok(mut q_result) => {
                    if let Some(row_result)  = q_result.next() {
                        let row = row_result?;
                        Some(mysql::from_row(row))
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
}

impl HandlesQuery<FindSurveysByAuthorQuery> for MysqlSurveyQueriesHandler {
    // String in this case resembles a Vec<SurveyDTO> but is just pure json string.
    type Result = Result<Option<String>, mysql::Error>;

    fn handle(&mut self, query: FindSurveysByAuthorQuery) -> Self::Result {
        // Default lower and upper bounds in case they aren't supplied in query object.
        let mut lower = 0;
        let mut upper = 20;
        if let Some(PageConfig{ page_num, page_size}) = &query.page_config {
            lower = (page_num - 1) * page_size;
            upper = page_num * page_size;
        }

        let survey_results: Option<String> =
            match self.conn.prep_exec(
                "SELECT survey_data FROM survey WHERE author=? LIMIT ?,?",
                (query.author, lower, upper)
            ) {
                Ok(mut q_result) => {
                    let mut s_dtos = Vec::new();
                    for row_result in q_result {
                        let row = row_result?;
                        let survey_data: String = mysql::from_row(row);
                        let s_dto: SurveyDTO = serde_json::from_str(&survey_data).unwrap();
                        s_dtos.push(s_dto);
                    }
                    if s_dtos.len() == 0 {
                        None
                    } else {
                        // TODO: Switch this to compact for real use - this is just to make output for testing friendlier.
                        Some(serde_json::to_string_pretty(&s_dtos).unwrap())
                    }
                },
                Err(e) => {
                    return Err(e);
                },
            };

        Ok(survey_results)
    }
}

impl HandlesQuery<SurveyQueries> for MysqlSurveyQueriesHandler {
    // The beautify of using a String for success is that we can coalesce all query handlers since they
    // now all have the same type signature.
    type Result = Result<Option<String>, mysql::Error>;

    fn handle(&mut self, query: SurveyQueries) -> Self::Result {
        match query {
            SurveyQueries::FindAuthorsSurveysQuery(q) => self.handle(q),
            SurveyQueries::FindSurveyQuery(q) => self.handle(q),
        }
    }
}