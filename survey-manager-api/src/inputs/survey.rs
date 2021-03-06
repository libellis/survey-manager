use serde::Deserialize;
use survey_manager_core::app_services::commands::{CreateSurveyCommand, CreateQuestionCommand, CreateChoiceCommand, UpdateSurveyCommand, PatchQuestion, PatchChoice};
use std::convert::{Into, TryInto};
use survey_manager_core::app_services::decode_payload;
use crate::error::TokenError;

#[derive(Deserialize)]
pub struct CreateSurveyDTO {
    pub token: String,
    pub title: String,
    pub description: String,
    pub category: String,
    pub questions: Vec<CreateQuestionDTO>,
}

#[derive(Deserialize)]
pub struct CreateQuestionDTO {
    #[serde(rename = "type")]
    pub question_type: String,
    pub title: String,
    pub choices: Vec<CreateChoiceDTO>
}

#[derive(Deserialize)]
pub struct CreateChoiceDTO {
    pub content: Option<String>,
    pub content_type: String,
    pub title: String,
}

impl TryInto<CreateSurveyCommand> for CreateSurveyDTO {
    type Error = TokenError;

    fn try_into(self) -> Result<CreateSurveyCommand, Self::Error> {
        let author = decode_payload(&self.token)
            .map_err(|_| TokenError::TokenExpired )?
            .username;

        let questions: Vec<CreateQuestionCommand> = self.questions
            .into_iter()
            .map(|q| {
                q.into()
            }).collect();

        Ok(CreateSurveyCommand {
            author,
            title: self.title,
            description: self.description,
            category: self.category,
            questions,
        })
    }
}

impl Into<CreateQuestionCommand> for CreateQuestionDTO {
    fn into(self) -> CreateQuestionCommand {
        let choices: Vec<CreateChoiceCommand> = self.choices
            .into_iter()
            .map(|c| {
                c.into()
            }).collect();

        CreateQuestionCommand {
            question_type: self.question_type,
            title: self.title,
            choices,
        }
    }
}

impl Into<CreateChoiceCommand> for CreateChoiceDTO {
    fn into(self) -> CreateChoiceCommand {
        CreateChoiceCommand {
            content: self.content,
            content_type: self.content_type,
            title: self.title,
        }
    }
}

#[derive(Deserialize)]
pub struct UpdateSurveyDTO {
    pub token: String,
    pub id: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub category: Option<String>,
    pub questions: Option<Vec<UpdateQuestionDTO>>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateQuestionDTO {
    pub id: Option<String>,
    #[serde(rename = "type")]
    pub question_type: Option<String>,
    pub title: Option<String>,
    pub choices: Option<Vec<UpdateChoiceDTO>>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateChoiceDTO {
    pub id: Option<String>,
    pub content: Option<Option<String>>,
    pub content_type: Option<String>,
    pub title: Option<String>,
}

impl TryInto<UpdateSurveyCommand> for UpdateSurveyDTO {
    type Error = TokenError;

    fn try_into(self) -> Result<UpdateSurveyCommand, Self::Error> {
        let author = decode_payload(&self.token)
            .map_err(|_| TokenError::TokenExpired )?
            .username;

        let questions = if let Some(q) = self.questions {
            Some(q.into_iter()
                .map(|q| {
                    q.into()
                }).collect())
        } else {
            None
        };

        Ok(UpdateSurveyCommand {
            id: self.id,
            author,
            title: self.title,
            description: self.description,
            category: self.category,
            questions,
        })
    }
}

impl Into<PatchQuestion> for UpdateQuestionDTO {
    fn into(self) -> PatchQuestion {
        let choices = if let Some(c) = self.choices {
            Some(c.into_iter()
                .map(|c| {
                    c.into()
                }).collect())
        } else {
            None
        };

        PatchQuestion {
            id: self.id,
            question_type: self.question_type,
            title: self.title,
            choices,
        }
    }
}

impl Into<PatchChoice> for UpdateChoiceDTO {
    fn into(self) -> PatchChoice {
        PatchChoice {
            id: self.id,
            content: self.content,
            content_type: self.content_type,
            title: self.title,
        }
    }
}
