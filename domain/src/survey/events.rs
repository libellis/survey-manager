use domain_patterns::event::DomainEvent;
use domain_patterns::message::Message;
use domain_patterns::models::Entity;
use uuid::Uuid;
use crate::survey::Survey;
use chrono::Utc;

#[derive(DomainEvent)]
pub struct SurveyCreatedEvent {
    id: Uuid,
    aggregate_id: String,
    version: u64,
    occurred: i64,
    author: String,
    title: String,
    description: String,
    created_on: i64,
    category: String,
    questions: Vec<QuestionCreatedEvent>
}

pub struct QuestionCreatedEvent {
    id: String,
    question_type: String,
    title: String,
    choices: Vec<ChoiceCreatedEvent>
}

pub struct ChoiceCreatedEvent {
    id: String,
    // Empty string if None.  Think about changing this?  not sure on this one.
    content: String,
    content_type: String,
    title: String,
}

impl From<&Survey> for SurveyCreatedEvent {
    fn from(survey: &Survey) -> Self {
        let questions: Vec<QuestionCreatedEvent> = survey.questions.iter().map(|q| {
            QuestionCreatedEvent {
                id: q.id.to_string(),
                question_type: q.question_type.to_string(),
                title: q.title().to_string(),
                choices: q.choices.iter().map(|c|{
                    let content = if let Some(c) = &c.content {
                        c.to_string()
                    } else {
                        "".to_string()
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
            id: Uuid::new_v4(),
            aggregate_id: survey.id(),
            version: survey.version(),
            occurred: Utc::now().timestamp(),
            author: survey.author.to_string(),
            title: survey.title.to_string(),
            description: survey.description.to_string(),
            created_on: survey.created_on,
            category: survey.category.to_string(),
            questions,
        }
    }
}

#[derive(DomainEvents)]
pub enum SurveyEvents {
    SurveyCreatedEvent(SurveyCreatedEvent)
}