use serde::Deserialize;
use crate::domain::survey::input::{NewSurveyIn, NewQuestionIn, NewChoiceIn};
use std::convert::Into;

#[derive(Deserialize)]
pub struct NewSurveyData {
    pub token: String,
    pub title: String,
    pub description: String,
    pub category: String,
    pub questions: Vec<NewQuestionData>,
}

#[derive(Deserialize)]
pub struct NewQuestionData {
    pub question_type: String,
    pub title: String,
    pub choices: Vec<NewChoiceData>
}

#[derive(Deserialize)]
pub struct NewChoiceData {
    pub content: Option<String>,
    pub content_type: String,
    pub title: String,
}

impl Into<NewSurveyIn> for NewSurveyData {
    fn into(self) -> NewSurveyIn {
        let questions: Vec<NewQuestionIn> = self.questions
            .into_iter()
            .map(|q| {
                q.into()
            }).collect();

        // TODO: Change to actually parse author from token once we implement token logic.
        let author = self.token;
        NewSurveyIn {
            author,
            title: self.title,
            description: self.description,
            category: self.category,
            questions,
        }
    }
}

impl Into<NewQuestionIn> for NewQuestionData {
    fn into(self) -> NewQuestionIn {
        let choices: Vec<NewChoiceIn> = self.choices
            .into_iter()
            .map(|c| {
                c.into()
            }).collect();

        NewQuestionIn {
            question_type: self.question_type,
            title: self.title,
            choices,
        }
    }
}

impl Into<NewChoiceIn> for NewChoiceData {
    fn into(self) -> NewChoiceIn {
        NewChoiceIn {
            content: self.content,
            content_type: self.content_type,
            title: self.title,
        }
    }
}
