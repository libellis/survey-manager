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

impl From<QuestionDTO> for Question {
    fn from(dto: QuestionDTO) -> Self {
        let choices: Vec<Choice> = dto.choices.into_iter()
            .map(|c| {
                Choice::from(c)
            }).collect();
        Question {
            id: Uuid::from_str(&dto.id).unwrap().clone(),
            question_type: QuestionType::try_from(dto.question_type).unwrap(),
            title: Title::try_from(dto.title).unwrap(),
            choices,
        }
    }
}

