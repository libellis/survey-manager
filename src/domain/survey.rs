use crate::domain::value_objects::Title;
use uuid::Uuid;
use crate::domain::{Question, SurveyEvents};
use domain_patterns::models::AggregateRoot;

#[derive(Entity)]
pub struct Survey {
    id: Uuid,
    version: u64,
    author: String,
    title: Title,
    description: String,
    date_posted: i64,
    category: String,
    questions: Vec<Question>,
}

impl AggregateRoot for Survey {
    type Events = SurveyEvents;
}