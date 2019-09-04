use uuid::Uuid;
use crate::domain::value_objects::{Title, QuestionType};
use crate::domain::Choice;

#[derive(Entity)]
pub struct Question {
    id: Uuid,
    version: u64,
    survey_id: Uuid,
    question_type: QuestionType,
    title: Title,
    choices: Vec<Choice>
}
