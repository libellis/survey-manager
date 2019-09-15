use uuid::Uuid;
use crate::value_objects::{QuestionType, Title};
use crate::survey::Choice;
use crate::dtos::{QuestionDTO, ChoiceDTO};
use std::str::FromStr;
use std::convert::TryFrom;

// TODO: Remove version from entities.  Only aggregate roots should have a version.
#[derive(Entity)]
pub struct Question {
    pub(super) id: Uuid,
    // TODO: Change to just type, weird to type question.question_type.  doesn't need to match db fields.
    pub(super) question_type: QuestionType,
    pub(super) title: Title,
    pub(super) choices: Vec<Choice>
}
