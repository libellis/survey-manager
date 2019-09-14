use domain_patterns::collections::Repository;
use survey_manager_core::survey::Survey;
use survey_manager_core::dtos::SurveyDTO;
use domain_patterns::models::Entity;
use mysql;
use mysql::from_row;

pub struct MysqlSurveyRepository {
    // A single connection to Mysql.  Handed down from a pool likely.
    conn: mysql::PooledConn,
}

// TODO: Build in transaction support.
impl Repository<Survey> for MysqlSurveyRepository {
    type Error = crate::Error;

    fn insert(&mut self, entity: &Survey) -> Result<Option<Survey>, Self::Error> {
        let survey_dto: SurveyDTO = entity.into();
        let survey_json = serde_json::to_string(&survey_dto).unwrap();
        // In this example survey_data is json of the entire survey object.  the other fields are just useful for query purposes and duplicate data.
        if let Err(_) = self.conn.prep_exec(
            "INSERT INTO surveys (id, version, author, title, category, created_on, survey_data) VALUES (?, ?, ?, ?, ?, ?, ?)",
            (entity.id(), entity.version(), entity.author().to_string(), entity.title().to_string(), entity.category().to_string(), entity.created_on(), survey_json)
        ) {
            // TODO: Assume error means duplicate key.  figure out what error actually means this for accurate translation.
            // We obviously need to be able to deal with other kinds of database errors.
            return Ok(None);
        };

        let survey: Survey = Survey::from(survey_dto);
        Ok(Some(survey))
    }

    fn get(&self, key: &String) -> Result<Option<Survey>, Self::Error> {
        unimplemented!()
    }

    fn get_paged(&self, page_num: usize, page_size: usize) -> Result<Vec<Survey>, Self::Error> {
        unimplemented!()
    }

    fn update(&mut self, entity: &Survey) -> Result<Option<Survey>, Self::Error> {
        unimplemented!()
    }

    fn remove(&mut self, key: &String) -> Result<Option<Survey>, Self::Error> {
        unimplemented!()
    }
}