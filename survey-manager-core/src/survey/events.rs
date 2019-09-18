use serde::Serialize;
use domain_patterns::event::DomainEvent;
use domain_patterns::message::Message;
use domain_patterns::models::{Entity, AggregateRoot};
use uuid::Uuid;
use crate::survey::Survey;
use chrono::Utc;
use crate::app_services::commands::UpdateSurveyCommand;

#[derive(DomainEvent, Serialize)]
pub struct SurveyCreatedEvent {
    pub id: String,
    pub aggregate_id: String,
    pub version: u64,
    pub occurred: i64,
    pub author: String,
    pub title: String,
    pub description: String,
    pub category: String,
    pub questions: Vec<QuestionCreatedEvent>
}

#[derive(Serialize)]
pub struct QuestionCreatedEvent {
    pub id: String,
    pub question_type: String,
    pub title: String,
    pub choices: Vec<ChoiceCreatedEvent>
}

#[derive(Serialize)]
pub struct ChoiceCreatedEvent {
    pub id: String,
    pub content: Option<String>,
    pub content_type: String,
    pub title: String,
}

impl From<&Survey> for SurveyCreatedEvent {
    fn from(survey: &Survey) -> Self {
        let questions: Vec<QuestionCreatedEvent> = survey.questions.iter().map(|q| {
            QuestionCreatedEvent {
                id: q.id.to_string(),
                question_type: q.kind.to_string(),
                title: q.title().to_string(),
                choices: q.choices.iter().map(|c|{
                    let content = if let Some(c) = &c.content {
                        Some(c.to_string())
                    } else {
                        None
                    };
                    ChoiceCreatedEvent {
                        id: c.id.to_string(),
                        content,
                        content_type: c.content_type.to_string(),
                        title: c.title.to_string(),
                    }
                }).collect(),
            }
        }).collect();

        SurveyCreatedEvent {
            id: Uuid::new_v4().to_string(),
            aggregate_id: survey.id(),
            version: survey.version(),
            occurred: survey.created_on,
            author: survey.author.to_string(),
            title: survey.title.to_string(),
            description: survey.description.to_string(),
            category: survey.category.to_string(),
            questions,
        }
    }
}

#[derive(DomainEvent, Serialize)]
pub struct SurveyUpdatedEvent {
    pub id: String,
    pub aggregate_id: String,
    pub version: u64,
    pub occurred: i64,
    pub title: Option<String>,
    pub description: Option<String>,
    pub category: Option<String>,
    pub questions: Option<Vec<QuestionUpdatedEvent>>,
}

#[derive(Serialize)]
pub struct QuestionUpdatedEvent {
    pub id: String,
    pub question_type: Option<String>,
    pub title: Option<String>,
    pub choices: Option<Vec<ChoiceUpdatedEvent>>,
}

#[derive(Serialize)]
pub struct ChoiceUpdatedEvent {
    pub id: String,
    // Empty string if None.  Think about changing this?  not sure on this one.
    pub content: Option<Option<String>>,
    pub content_type: Option<String>,
    pub title: Option<String>,
}

#[derive(DomainEvents)]
pub enum SurveyEvents {
    SurveyCreatedEvent(SurveyCreatedEvent),
    SurveyUpdatedEvent(SurveyUpdatedEvent),
}
