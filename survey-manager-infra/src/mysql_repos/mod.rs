lazy_static! {
    static ref MYSQL_POOL: mysql::Pool = {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        mysql::Pool::new(&database_url).unwrap()
    };
}

pub mod mysql_survey_repository;
pub use mysql_survey_repository::*;

pub mod mysql_surveydto_read_repository;
pub use mysql_surveydto_read_repository::*;
