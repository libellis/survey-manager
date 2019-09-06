use serde::Serialize;
use std::convert::Into;
use domain_patterns::models::Entity;
use crate::domain::survey::{Choice, Survey, Question};

#[derive(Serialize)]
pub struct SurveyCreated {
    pub id: String,
    pub version: u64,
    pub author: String,
    pub title: String,
    pub description: String,
    // TODO: Change into nice timestamp.
    pub created_on: i64,
    pub category: String,
    pub questions: Vec<QuestionCreated>,
}

#[derive(Serialize)]
pub struct QuestionCreated {
    pub id: String,
    pub version: u64,
    pub question_type: String,
    pub title: String,
    pub choices: Vec<ChoiceCreated>
}

#[derive(Serialize)]
pub struct ChoiceCreated {
    pub id: String,
    pub version: u64,
    pub content: Option<String>,
    pub content_type: String,
    pub title: String,
}

impl From<Survey> for SurveyCreated {
    fn from(s: Survey) -> Self {
        let questions: Vec<QuestionCreated> = s.questions()
            .into_iter()
            .map(|q| QuestionCreated::from(q))
            .collect();
        
        SurveyCreated {
            id: s.id().to_string(),
            version: s.version(),
            author: s.author().clone(),
            title: s.title().to_string(),
            description: s.description().to_string(),
            created_on: s.created_on().clone(),
            category: s.category().clone(),
            questions: vec![]
        }
    }
}

impl From<&Question> for QuestionCreated {
    fn from(q: &Question) -> Self {
        let choices = q.choices()
            .into_iter()
            .map(|c| ChoiceCreated::from(c))
            .collect();

        QuestionCreated {
            id: q.id().to_string(),
            version: q.version(),
            question_type: q.question_type().to_string(),
            title: q.title().to_string(),
            choices,
        }
    }
}

impl From<&Choice> for ChoiceCreated {
    fn from(choice: &Choice) -> Self {
        let content = if let Some(c) = choice.content() {
            Some(c.to_string())
        } else {
            None
        };

        ChoiceCreated {
            id: choice.id().to_string(),
            version: choice.version(),
            content,
            content_type: choice.content_type().to_string(),
            title: choice.title().to_string(),
        }
    }
}


