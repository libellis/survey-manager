use domain_patterns::collections::Repository;
use survey_manager_core::survey::Survey;
use survey_manager_core::dtos::SurveyDTO;
use domain_patterns::models::Entity;
use mysql;
use mysql::Error;
use mysql::error::ServerError;
use mysql::from_row;

pub struct MysqlSurveyRepository {
    // A single connection to Mysql.  Handed down from a pool likely.
    conn: mysql::PooledConn,
}

impl MysqlSurveyRepository {
    pub fn new(conn: mysql::PooledConn) -> MysqlSurveyRepository {
        MysqlSurveyRepository {
            conn,
        }
    }
}

// TODO: Build in transaction support.
impl Repository<Survey> for MysqlSurveyRepository {
    type Error = mysql::Error;

    fn insert(&mut self, entity: &Survey) -> Result<Option<String>, Self::Error> {
        let survey_dto: SurveyDTO = entity.into();
        let survey_json = serde_json::to_string(&survey_dto).unwrap();

        // In this example survey_data is json of the entire survey object.  the other fields are just useful for query purposes and duplicate data.
        if let Err(e) = self.conn.prep_exec(
            "INSERT INTO survey (id, version, author, title, category, created_on, survey_data) VALUES (?, ?, ?, ?, ?, ?, ?)",
            (entity.id(), entity.version(), entity.author().to_string(), entity.title().to_string(), entity.category().to_string(), entity.created_on(), survey_json)
        ) {
            return handle_duplicate_key(e);
        };

        // Success.  Return the PK back as is.
        Ok(Some(survey_dto.id))
    }

    // TODO: Change to not use prepared statements when we switch to vitess.  Likely do this
    // when you have a vitess client library built and it's trivial.
    fn get(&mut self, key: &String) -> Result<Option<Survey>, Self::Error> {
        let survey_result: Option<SurveyDTO> =
            match self.conn.prep_exec(
            "SELECT survey_data FROM survey WHERE id=?",
            (key,)
        ) {
            Ok(mut q_result) => {
                if let Some(row_result)  = q_result.next() {
                    let row = row_result?;
                    let survey_data: String = mysql::from_row(row);
                    // TODO: Rather than using mysql-simple error type, create your own and attach theirs to yours
                    // so you can include serde serialization errors.
                    serde_json::from_str(&survey_data).unwrap()
                } else {
                    None
                }
            },
            Err(e) => {
                return Err(e);
            },
        };

        if let Some(survey_dto) = survey_result {
            return Ok(Some(survey_dto.into()))
        }

        Ok(None)
    }

    // Intentionally leaving this unimplemented.  we don't need it for command side.
    fn get_paged(&mut self, page_num: usize, page_size: usize) -> Result<Vec<Survey>, Self::Error> {
        unimplemented!()
    }

    fn update(&mut self, entity: &Survey) -> Result<Option<String>, Self::Error> {
        let survey_dto: SurveyDTO = entity.into();
        let survey_json = serde_json::to_string(&survey_dto).unwrap();

        // In this example survey_data is json of the entire survey object.  the other fields are just useful for query purposes and duplicate data.
        match self.conn.prep_exec(
            "UPDATE survey SET version = ?, title = ?, category = ?, survey_data = ? WHERE id = ?",
            (entity.version(), entity.title().to_string(), entity.category().to_string(), survey_json, entity.id())
        ) {
            Ok(result) => {
                if result.affected_rows() == 0 {
                    return Ok(None);
                }
            },
            Err(e) => {
                return Err(e);
            }
        };

        // Success.  Return the PK back as is.
        Ok(Some(survey_dto.id))
    }

    fn remove(&mut self, key: &String) -> Result<Option<String>, Self::Error> {
        match self.conn.prep_exec(
            "DELETE FROM survey WHERE id = ?",
            (key,)
        ) {
            Ok(result) => {
                if result.affected_rows() == 0 {
                    return Ok(None);
                }
            },
            Err(e) => {
                return Err(e);
            }
        };

        // Success.  Return the PK back as is.
        Ok(Some(key.clone()))
    }
}

fn handle_duplicate_key(error: mysql::Error) -> Result<Option<String>, mysql::Error> {
    if let Error::MySqlError(e) = error {
        // TODO: Make sure this is the right error code for valid primary key column name
        // in WHERE clause, but id supplied does not exist.
        if e.code == ServerError::ER_DUP_ENTRY as u16 {
            return Ok(None);
        }
        // Some other code, so return the error.
        // TODO: Add ways to deal with other errors as we actually enounter them.
        return Err(Error::MySqlError(e))
    }

    // TODO: Add ways to deal with other errors as we actually enounter them.
    return Err(error);
}
