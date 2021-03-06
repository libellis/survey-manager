use survey_manager_core::app_services::queries::SurveyQueries;
use domain_patterns::query::HandlesQuery;
use futures::Future;
use actix_web::web;
use crate::generate;
use crate::error::ApiError;

pub fn handle_queries_async(
    query: SurveyQueries,
) -> impl Future<Item = String, Error = ApiError> {
    web::block(move || generate::query_handler().handle(query) )
        .from_err()
}
