use survey_manager_core::app_services::queries::SurveyQueries;
use domain_patterns::query::HandlesQuery;
use futures::Future;
use actix_web::{web, Error as AWError};
use crate::generate;
use crate::error::Error;

pub fn handle_queries_async(
    query: SurveyQueries,
) -> impl Future<Item = Option<String>, Error = AWError> {
    web::block(move || handle(query))
        .from_err()
}

fn handle(
    query: SurveyQueries,
) -> Result<Option<String>, Error> {
    generate::query_handler().handle(query)
        .map_err(|e| Error::from(e))
}

pub fn handle_queries_async_no_cache(
    query: SurveyQueries,
) -> impl Future<Item = Option<String>, Error = AWError> {
    web::block(move || handle_no_cache(query))
        .from_err()
}

fn handle_no_cache(
    query: SurveyQueries,
) -> Result<Option<String>, Error> {
    generate::query_handler_no_cache().handle(query)
        .map_err(|e| Error::from(e))
}
