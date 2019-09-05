use serde::Deserialize;
use crate::domain::survey::input::{NewSurvey, NewQuestion, NewChoice};
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

impl Into<NewSurvey> for NewSurveyData {
    fn into(self) -> NewSurvey {
        let questions: Vec<NewQuestion> = self.questions
            .into_iter()
            .map(|q| {
                q.into()
            }).collect();

        // TODO: Change to actually parse author from token once we implement token logic.
        let author = self.token;
        NewSurvey {
            author,
            title: self.title,
            description: self.description,
            category: self.category,
            questions,
        }
    }
}

impl Into<NewQuestion> for NewQuestionData {
    fn into(self) -> NewQuestion {
        let choices: Vec<NewChoice> = self.choices
            .into_iter()
            .map(|c| {
                c.into()
            }).collect();

        NewQuestion {
            question_type: self.question_type,
            title: self.title,
            choices,
        }
    }
}

impl Into<NewChoice> for NewChoiceData {
    fn into(self) -> NewChoice {
        NewChoice {
            content: self.content,
            content_type: self.content_type,
            title: self.title,
        }
    }
}
