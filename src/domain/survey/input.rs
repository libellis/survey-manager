pub struct NewSurveyIn {
    pub author: String,
    pub title: String,
    pub description: String,
    pub category: String,
    pub questions: Vec<NewQuestionIn>,
}

pub struct NewQuestionIn {
    pub question_type: String,
    pub title: String,
    pub choices: Vec<NewChoiceIn>
}

pub struct NewChoiceIn {
    pub content: Option<String>,
    pub content_type: String,
    pub title: String,
}
