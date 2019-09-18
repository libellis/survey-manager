use domain_patterns::query::Query;

#[derive(Query)]
pub struct FindSurveysByAuthorQuery {
    pub author: String,
    pub page_config: Option<PageConfig>,
}

pub struct PageConfig {
    pub page_size: usize,
    pub page_num: usize,
}
