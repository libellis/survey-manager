use uuid::Uuid;
use crate::value_objects::{QuestionType, Title};
use crate::survey::Choice;
use crate::dtos::{QuestionDTO, ChoiceDTO};
use std::str::FromStr;
use std::convert::TryFrom;

#[derive(Entity)]
pub struct Question {
    pub(super) id: Uuid,
    pub(super) kind: QuestionType,
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
            kind: QuestionType::try_from(dto.kind).unwrap(),
            title: Title::try_from(dto.title).unwrap(),
            choices,
        }
    }
}

