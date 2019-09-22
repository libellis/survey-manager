use survey_manager_core::app_services::queries::SurveyQueries;
use domain_patterns::query::HandlesQuery;
use futures::Future;
use actix_web::{web, Error as AWError};
use crate::generate;
use crate::error::CoreError;
use actix_web::error::BlockingError;

pub fn handle_queries_async(
    query: SurveyQueries,
) -> impl Future<Item = String, Error = AWError> {
    web::block(move || generate::query_handler().handle(query) )
        .map_err(|e| {
            match e {
                BlockingError::Error(e) => CoreError(e),
                _ => CoreError::thread_fail(),
            }
        })
        .from_err()
}
