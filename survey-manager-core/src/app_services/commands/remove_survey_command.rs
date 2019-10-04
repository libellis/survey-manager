use domain_patterns::command::Command;
use domain_patterns::message::Message;

#[derive(Clone, Command)]
pub struct RemoveSurveyCommand {
    pub id: String,
    pub requesting_author: String,
}
