use domain_patterns::query::Query;

#[derive(Query)]
pub struct FindSurveyQuery {
    pub id: String,
}