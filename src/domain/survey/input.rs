pub struct NewSurveyOut {
    pub author: String,
    pub title: String,
    pub description: String,
    pub category: String,
    pub questions: Vec<NewQuestionOut>,
}

pub struct NewQuestionOut {
    pub question_type: String,
    pub title: String,
    pub choices: Vec<NewChoiceOut>
}

pub struct NewChoiceOut {
    pub content: Option<String>,
    pub content_type: String,
    pub title: String,
}
