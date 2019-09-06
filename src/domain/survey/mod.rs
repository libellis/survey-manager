pub mod choice;
pub use choice::*;

pub mod question;
pub use question::*;

pub mod events;
pub use events::*;

/// Input module provides input argument structures for survey aggregate root
/// constructor.
pub mod input;

use crate::domain::value_objects::{Title, QuestionType, ContentType};
use uuid::Uuid;
use domain_patterns::models::{AggregateRoot};
use std::error::Error;
use std::convert::TryFrom;
use chrono::Utc;
use crate::domain::survey::input::{NewSurveyOut, NewQuestionOut, NewChoiceOut};

#[derive(Entity)]
pub struct Survey {
    id: Uuid,
    version: u64,
    author: String,
    title: Title,
    description: String,
    // TODO: Change into a nice timestamp.
    created_on: i64,
    category: String,
    questions: Vec<Question>,
}

impl Survey {
    pub fn new(
        new_survey: NewSurveyOut,
    ) -> Result<Survey, Box<dyn Error>> {
        Ok(Survey {
            id: Uuid::new_v4(),
            version: 0,
            author: new_survey.author,
            title: Title::try_from(new_survey.title)?,
            description: new_survey.description,
            created_on: Utc::now().timestamp(),
            category: new_survey.category,
            questions: Self::create_questions(new_survey.questions)?
        })
    }

    fn create_questions(new_questions: Vec<NewQuestionOut>) -> Result<Vec<Question>, Box<dyn Error>> {
        let q_results = new_questions
            .into_iter()
            .map(|q| { Self::create_question(q) });

        let mut questions: Vec<_> = vec![];

        for q_result in q_results {
            questions.push(q_result?)
        }

        Ok(questions)
    }

    fn create_question(new_question: NewQuestionOut) -> Result<Question, Box<dyn Error>> {
        Ok(Question {
            id: Uuid::new_v4(),
            version: 0,
            question_type: QuestionType::try_from(new_question.question_type)?,
            title: Title::try_from(new_question.title)?,
            choices: Self::create_choices(new_question.choices)?,
        })
    }

    fn create_choices(new_choices: Vec<NewChoiceOut>) -> Result<Vec<Choice>, Box<dyn Error>> {
        let c_results = new_choices
            .into_iter()
            .map(|c| { Self::create_choice(c) });

        let mut choices: Vec<_> = vec![];

        for c_result in c_results {
            choices.push(c_result?)
        }

        Ok(choices)
    }

    fn create_choice(new_choice: NewChoiceOut) -> Result<Choice, Box<dyn Error>> {
        Ok(Choice {
            id: Uuid::new_v4(),
            version: 0,
            // TODO: Update for content translation once we understand embed strings.
            content: None,
            content_type: ContentType::try_from(new_choice.content_type)?,
            title: Title::try_from(new_choice.title)?,
        })
    }
}

impl AggregateRoot for Survey {
    type Events = SurveyEvents;
}
