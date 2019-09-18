use domain_patterns::query::{Query, HandlesQuery};
use crate::app_services::queries::{FindSurveysByAuthorQuery, FindSurveyQuery, PageConfig};
use crate::app_services::repository_contracts::SurveyDTOReadRepository;
use crate::Error;
use crate::errors::Error::RepoFailure;

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

pub struct SurveyQueriesHandler<T>
    where T: SurveyDTOReadRepository
{
    repo: T
}

impl<T> SurveyQueriesHandler<T>
    where T: SurveyDTOReadRepository
{
    pub fn new(repo: T) -> SurveyQueriesHandler<T> {
        SurveyQueriesHandler {
            repo,
        }
    }
}

impl<T> HandlesQuery<FindSurveyQuery> for SurveyQueriesHandler<T>
    where T: SurveyDTOReadRepository
{
    // String in this case is just the pure JSON.
    // no need to turn it into a data structure - we are just giving the caller
    // json anyways.
    type Result = Result<Option<String>, Error>;

    fn handle(&mut self, query: FindSurveyQuery) -> Self::Result {
        let results = self.repo
            .get_survey_for_author(&query.id, &query.requesting_author)
            .map_err(|e| RepoFailure { source: Box::new(e) })?;

        if let Some(survey) = results {
            return Ok(Some(serde_json::to_string(&survey).unwrap()));
        }

        Ok(None)
    }
}

impl<T> HandlesQuery<FindSurveysByAuthorQuery> for SurveyQueriesHandler<T>
    where T: SurveyDTOReadRepository
{
    // String in this case resembles a Vec<SurveyDTO> but is just pure json string.
    type Result = Result<Option<String>, Error>;

    fn handle(&mut self, query: FindSurveysByAuthorQuery) -> Self::Result {
        // Default lower and upper bounds in case they aren't supplied in query object.
        let mut lower = 0;
        let mut upper = 20;
        if let Some(PageConfig{ page_num, page_size}) = &query.page_config {
            lower = (page_num - 1) * page_size;
            upper = page_num * page_size;
        }

        let results = self.repo
            .get_surveys_by_author(&query.author, lower, upper)
            .map_err(|e| RepoFailure { source: Box::new(e) })?;

        if let Some(surveys) = results {
            return Ok(Some(serde_json::to_string(&surveys).unwrap()));
        }

        Ok(None)
    }
}

impl<T> HandlesQuery<SurveyQueries> for SurveyQueriesHandler<T>
    where T: SurveyDTOReadRepository
{
    // The beautify of using a String for success is that we can coalesce all query handlers since they
    // now all have the same type signature.
    type Result = Result<Option<String>, Error>;

    fn handle(&mut self, query: SurveyQueries) -> Self::Result {
        match query {
            SurveyQueries::FindAuthorsSurveysQuery(q) => self.handle(q),
            SurveyQueries::FindSurveyQuery(q) => self.handle(q),
        }
    }
}
