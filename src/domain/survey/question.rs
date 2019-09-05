use uuid::Uuid;
use crate::domain::value_objects::{QuestionType, Title};
use crate::domain::survey::Choice;

#[derive(Entity)]
pub struct Question {
    pub(super) id: Uuid,
    pub(super) version: u64,
    pub(super) question_type: QuestionType,
    pub(super) title: Title,
    pub(super) choices: Vec<Choice>
}
