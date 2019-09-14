use domain_patterns::collections::Repository;
use survey_manager_core::survey::Survey;
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
        // In this example survey_data is json of the entire survey object.  the other fields are just useful for query purposes and duplicate data.
//        self.conn.prep_exec(
//            "INSERT INTO surveys (id, version, author, title, category, created_on, survey_data) VALUES (?, ?, ?, ?, ?, ?, ?)",
//            (entity.id(), entity.version(), entity.author(), entity.title(), entity.category(), entity.created_on(), entity)
//        );
        unimplemented!()
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