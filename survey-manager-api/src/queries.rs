use survey_manager_core::app_services::queries::SurveyQueries;
use domain_patterns::query::{HandlesQuery, Query};
use futures::Future;
use actix_web::{web, Error as AWError};
use crate::generate;
use survey_manager_core::Error;

pub type Pool = mysql::Pool;
pub type Conn = mysql::PooledConn;

pub fn handle_queries_async(
    pool: &Pool,
    query: SurveyQueries,
) -> impl Future<Item = Option<String>, Error = AWError> {
    let pool = pool.clone();
    web::block(move || handle(pool.get_conn().unwrap(), query))
        .from_err()
}

fn handle(
    conn: Conn,
    query: SurveyQueries,
) -> Result<Option<String>, String> {
    generate::query_handler(conn).handle(query)
        .map_err(|e| format!("{}", e))
}
