use domain_patterns::query::Query;
use crate::app_services::queries::{FindAuthorsSurveysQuery, FindSurveyQuery};

#[derive(Query)]
pub enum SurveyQueries {
    FindSurveyQuery(FindSurveyQuery),
    FindAuthorsSurveysQuery(FindAuthorsSurveysQuery),
}

impl From<FindSurveyQuery> for SurveyQueries {
    fn from(query: FindSurveyQuery) -> Self {
        SurveyQueries::FindSurveyQuery(query)
    }
}

impl From<FindAuthorsSurveysQuery> for SurveyQueries {
    fn from(query: FindAuthorsSurveysQuery) -> Self {
        SurveyQueries::FindAuthorsSurveysQuery(query)
    }
}
