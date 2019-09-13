use serde::Serialize;
use std::convert::From;
use domain_patterns::models::Entity;
use crate::survey::{Choice, Survey, Question};

#[derive(Serialize)]
pub struct SurveyDTO {
    pub id: String,
    pub version: u64,
    pub author: String,
    pub title: String,
    pub description: String,
    pub created_on: i64,
    pub category: String,
    pub questions: Vec<QuestionDTO>,
}

#[derive(Serialize)]
pub struct QuestionDTO {
    pub id: String,
    pub version: u64,
    pub question_type: String,
    pub title: String,
    pub choices: Vec<ChoiceDTO>
}

#[derive(Serialize)]
pub struct ChoiceDTO {
    pub id: String,
    pub version: u64,
    pub content: Option<String>,
    pub content_type: String,
    pub title: String,
}

impl From<Survey> for SurveyDTO {
    fn from(s: Survey) -> Self {
        let questions: Vec<QuestionDTO> = s.questions()
            .into_iter()
            .map(|q| QuestionDTO::from(q))
            .collect();
        
        SurveyDTO {
            id: s.id().to_string(),
            version: s.version(),
            author: s.author().to_string(),
            title: s.title().to_string(),
            description: s.description().to_string(),
            created_on: s.created_on().clone(),
            category: s.category().to_string(),
            questions,
        }
    }
}

impl From<&Question> for QuestionDTO {
    fn from(q: &Question) -> Self {
        let choices = q.choices()
            .into_iter()
            .map(|c| ChoiceDTO::from(c))
            .collect();

        QuestionDTO {
            id: q.id().to_string(),
            version: q.version(),
            question_type: q.question_type().to_string(),
            title: q.title().to_string(),
            choices,
        }
    }
}

impl From<&Choice> for ChoiceDTO {
    fn from(choice: &Choice) -> Self {
        let content = if let Some(c) = choice.content() {
            Some(c.to_string())
        } else {
            None
        };

        ChoiceDTO {
            id: choice.id().to_string(),
            version: choice.version(),
            content,
            content_type: choice.content_type().to_string(),
            title: choice.title().to_string(),
        }
    }
}


