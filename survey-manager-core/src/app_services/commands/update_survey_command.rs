use domain_patterns::command::Command;
use domain_patterns::message::Message;
use crate::app_services::commands::{CreateQuestionCommand, CreateChoiceCommand};
use std::convert::TryInto;
use crate::errors::Error;
use crate::value_objects::ValidationError::{ContentValidationError, ContentTypeValidationError, TitleValidationError, QuestionTypeValidationError, MissingChoicesError};

#[derive(Clone, Command)]
pub struct UpdateSurveyCommand {
    pub id: String,
    pub author: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub category: Option<String>,
    pub questions: Option<Vec<PatchQuestion>>,
}

// TODO: Reconsider naming since this could either patch an existing question or create a new question
// for a patched Survey.
#[derive(Clone)]
pub struct PatchQuestion {
    pub id: Option<String>,
    pub question_type: Option<String>,
    pub title: Option<String>,
    pub choices: Option<Vec<PatchChoice>>,
}

// TODO: Reconsider naming since this could either patch an existing choice or create a new choice
// for a patched Survey.
#[derive(Clone)]
pub struct PatchChoice {
    pub id: Option<String>,
    pub content: Option<Option<String>>,
    pub content_type: Option<String>,
    pub title: Option<String>,
}

impl TryInto<CreateQuestionCommand> for PatchQuestion {
    type Error = Error;

    fn try_into(self) -> Result<CreateQuestionCommand, Error> {
        let choices = self.choices
            .ok_or(MissingChoicesError)?
            .into_iter()
            .map(|c| {
                // try_into returns a result.  Due to control flow restrictions we can't use
                // ? operator inside a closure, so we use collect to transform a Vec<Result<CreateChoiceCommand, Error>>
                // into a Result<Vec<CreateChoiceCommand, Error>> and call ? outside of closure to
                // pass back error if exists, or get Vec<CreateChoiceCommand> out of result.
                c.try_into()
            }).collect::<Result<Vec<_>, _>>()?;

        Ok(
            CreateQuestionCommand {
                question_type: self.question_type.ok_or(QuestionTypeValidationError)?,
                title: self.title.ok_or(TitleValidationError { msg: "Missing title for new question, or trying to update question without supplying it's id.".to_string() })?,
                choices,
            }
        )
    }
}

impl TryInto<CreateChoiceCommand> for PatchChoice {
    type Error = Error;

    fn try_into(self) -> Result<CreateChoiceCommand, Error> {
        Ok(
            CreateChoiceCommand {
                content: self.content.ok_or(ContentValidationError)?,
                content_type: self.content_type.ok_or(ContentTypeValidationError)?,
                title: self.title.ok_or(TitleValidationError { msg: "Missing title for new choice, or trying to update choice without supplying it's id.".to_string() })?,
            }
        )
    }
}