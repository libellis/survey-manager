use domain_patterns::command::Command;
use domain_patterns::message::Message;

#[derive(Clone, Command)]
pub struct CreateSurveyCommand {
    pub author: String,
    pub title: String,
    pub description: String,
    pub category: String,
    pub questions: Vec<CreateQuestionCommand>,
}

// Sub-commands don't get considered commands in and of themselves.
#[derive(Clone)]
pub struct CreateQuestionCommand {
    pub question_type: String,
    pub title: String,
    pub choices: Vec<CreateChoiceCommand>
}

#[derive(Clone)]
pub struct CreateChoiceCommand {
    pub content: Option<String>,
    pub content_type: String,
    pub title: String,
}

pub struct SurveyChangeset {
    pub title: Option<String>,
    pub description: Option<String>,
    pub category: Option<String>,
    pub questions: Option<Vec<QuestionChangeset>>,
}

pub struct QuestionChangeset {
    pub id: String,
    pub question_type: Option<String>,
    pub title: Option<String>,
    pub choices: Option<Vec<ChoiceChangeset>>,
}

pub struct ChoiceChangeset {
    pub id: String,
    pub content: Option<Option<String>>,
    pub content_type: Option<String>,
    pub title: Option<String>,
}

#[derive(Command)]
pub enum SurveyCommands {
    CreateSurveyCommand(CreateSurveyCommand)
}
