use serde::Deserialize;
use crate::domain::survey::input::{NewSurveyData, NewQuestionData, NewChoiceData, SurveyChangeset, QuestionChangeset, ChoiceChangeset};
use std::convert::Into;
use crate::application::services::decode_payload;

#[derive(Deserialize)]
pub struct CreateSurveyCommand {
    pub token: String,
    pub title: String,
    pub description: String,
    pub category: String,
    pub questions: Vec<CreateQuestion>,
}

#[derive(Deserialize)]
pub struct CreateQuestion {
    pub question_type: String,
    pub title: String,
    pub choices: Vec<CreateChoice>
}

#[derive(Deserialize)]
pub struct CreateChoice {
    pub content: Option<String>,
    pub content_type: String,
    pub title: String,
}

impl Into<NewSurveyData> for CreateSurveyCommand {
    fn into(self) -> NewSurveyData {
        let author = decode_payload(&self.token).username;

        let questions: Vec<NewQuestionData> = self.questions
            .into_iter()
            .map(|q| {
                q.into()
            }).collect();

        NewSurveyData {
            author,
            title: self.title,
            description: self.description,
            category: self.category,
            questions,
        }
    }
}

impl Into<NewQuestionData> for CreateQuestion {
    fn into(self) -> NewQuestionData {
        let choices: Vec<NewChoiceData> = self.choices
            .into_iter()
            .map(|c| {
                c.into()
            }).collect();

        NewQuestionData {
            question_type: self.question_type,
            title: self.title,
            choices,
        }
    }
}

impl Into<NewChoiceData> for CreateChoice {
    fn into(self) -> NewChoiceData {
        NewChoiceData {
            content: self.content,
            content_type: self.content_type,
            title: self.title,
        }
    }
}

#[derive(Deserialize)]
pub struct UpdateSurveyCommand {
    pub token: String,
    pub id: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub category: Option<String>,
    pub questions: Option<Vec<UpdateQuestionCommand>>,
}

#[derive(Deserialize)]
pub struct UpdateQuestionCommand {
    pub id: String,
    pub question_type: Option<String>,
    pub title: Option<String>,
    pub choices: Option<Vec<UpdateChoiceCommand>>,
}

#[derive(Deserialize)]
pub struct UpdateChoiceCommand {
    pub id: String,
    pub content: Option<Option<String>>,
    pub content_type: Option<String>,
    pub title: Option<String>,
}

impl Into<SurveyChangeset> for UpdateSurveyCommand {
    fn into(self) -> SurveyChangeset {
        let questions = if let Some(q) = self.questions {
            Some(q.into_iter()
                .map(|q| {
                    q.into()
                }).collect())
        } else {
            None
        };

        SurveyChangeset {
            title: self.title,
            description: self.description,
            category: self.category,
            questions,
        }
    }
}

impl Into<QuestionChangeset> for UpdateQuestionCommand {
    fn into(self) -> QuestionChangeset {
        let choices = if let Some(c) = self.choices {
            Some(c.into_iter()
                .map(|c| {
                    c.into()
                }).collect())
        } else {
            None
        };

        QuestionChangeset {
            id: self.id,
            question_type: self.question_type,
            title: self.title,
            choices,
        }
    }
}

impl Into<ChoiceChangeset> for UpdateChoiceCommand {
    fn into(self) -> ChoiceChangeset {
        ChoiceChangeset {
            id: self.id,
            content: self.content,
            content_type: self.content_type,
            title: self.title,
        }
    }
}
