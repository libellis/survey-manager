use domain_patterns::query::Query;
use crate::app_services::queries::{FindSurveysByAuthorQuery, FindSurveyQuery};

#[derive(Query)]
pub enum SurveyQueries {
    FindSurveyQuery(FindSurveyQuery),
    FindAuthorsSurveysQuery(FindSurveysByAuthorQuery),
}

impl From<FindSurveyQuery> for SurveyQueries {
    fn from(query: FindSurveyQuery) -> Self {
        SurveyQueries::FindSurveyQuery(query)
    }
}

impl From<FindSurveysByAuthorQuery> for SurveyQueries {
    fn from(query: FindSurveysByAuthorQuery) -> Self {
        SurveyQueries::FindAuthorsSurveysQuery(query)
    }
}
