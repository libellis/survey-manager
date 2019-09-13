use domain_patterns::command::Command;
use domain_patterns::message::Message;

#[derive(Clone, Command)]
pub struct UpdateSurveyCommand {
    pub id: String,
    pub author: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub category: Option<String>,
    pub questions: Option<Vec<UpdateQuestionCommand>>,
}

#[derive(Clone)]
pub struct UpdateQuestionCommand {
    pub id: String,
    pub question_type: Option<String>,
    pub title: Option<String>,
    pub choices: Option<Vec<UpdateChoiceCommand>>,
}

#[derive(Clone)]
pub struct UpdateChoiceCommand {
    pub id: String,
    pub content: Option<Option<String>>,
    pub content_type: Option<String>,
    pub title: Option<String>,
}
