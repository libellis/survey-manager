use serde::Serialize;
use std::convert::From;
use domain_patterns::models::Entity;
use crate::domain::survey::{Choice, Survey, Question};

#[derive(Serialize)]
pub struct SurveyOut {
    pub id: String,
    pub version: u64,
    pub author: String,
    pub title: String,
    pub description: String,
    // TODO: Change into nice timestamp.
    pub created_on: i64,
    pub category: String,
    pub questions: Vec<QuestionOut>,
}

#[derive(Serialize)]
pub struct QuestionOut {
    pub id: String,
    pub version: u64,
    pub question_type: String,
    pub title: String,
    pub choices: Vec<ChoiceOut>
}

#[derive(Serialize)]
pub struct ChoiceOut {
    pub id: String,
    pub version: u64,
    pub content: Option<String>,
    pub content_type: String,
    pub title: String,
}

impl From<Survey> for SurveyOut {
    fn from(s: Survey) -> Self {
        let questions: Vec<QuestionOut> = s.questions()
            .into_iter()
            .map(|q| QuestionOut::from(q))
            .collect();
        
        SurveyOut {
            id: s.id().to_string(),
            version: s.version(),
            author: s.author().clone(),
            title: s.title().to_string(),
            description: s.description().to_string(),
            created_on: s.created_on().clone(),
            category: s.category().clone(),
            questions,
        }
    }
}

impl From<&Question> for QuestionOut {
    fn from(q: &Question) -> Self {
        let choices = q.choices()
            .into_iter()
            .map(|c| ChoiceOut::from(c))
            .collect();

        QuestionOut {
            id: q.id().to_string(),
            version: q.version(),
            question_type: q.question_type().to_string(),
            title: q.title().to_string(),
            choices,
        }
    }
}

impl From<&Choice> for ChoiceOut {
    fn from(choice: &Choice) -> Self {
        let content = if let Some(c) = choice.content() {
            Some(c.to_string())
        } else {
            None
        };

        ChoiceOut {
            id: choice.id().to_string(),
            version: choice.version(),
            content,
            content_type: choice.content_type().to_string(),
            title: choice.title().to_string(),
        }
    }
}


