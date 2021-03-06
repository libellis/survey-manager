use serde::{Serialize, Deserialize};
use std::convert::From;
use domain_patterns::models::{Entity, AggregateRoot};
use crate::survey::{Choice, Survey, Question};

#[derive(Serialize, Deserialize)]
pub struct SurveyDTOs {
    pub surveys: Vec<ListViewSurveyDTO>
}

#[derive(Serialize, Deserialize)]
pub struct ListViewSurveyDTO {
    pub id: String,
    pub author: String,
    pub title: String,
    pub category: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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

#[derive(Serialize, Deserialize)]
pub struct QuestionDTO {
    pub id: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub title: String,
    pub choices: Vec<ChoiceDTO>
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChoiceDTO {
    pub id: String,
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
            id: s.id(),
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
            kind: q.kind().to_string(),
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
            content,
            content_type: choice.content_type().to_string(),
            title: choice.title().to_string(),
        }
    }
}

// Same but from reference type, so needs clone.
impl From<&Survey> for SurveyDTO {
    fn from(s: &Survey) -> Self {
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

impl SurveyDTOs {
    pub fn into_bounded(mut self, lower_bound: usize, upper_bound: usize) -> Option<SurveyDTOs> {
        let total = self.surveys.len();
        if lower_bound > total - 1 {
            None
        } else {
            let start = lower_bound;
            let end = if upper_bound > total {
                total
            } else {
                upper_bound
            };

            self.surveys = self.surveys.drain(start..end).collect();

            Some(
                self
            )
        }
    }
}