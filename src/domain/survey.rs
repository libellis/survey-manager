use crate::domain::value_objects::Title;
use uuid::Uuid;
use crate::domain::Question;

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