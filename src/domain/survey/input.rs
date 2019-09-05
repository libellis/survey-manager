pub struct NewSurveyReq {
    pub author: String,
    pub title: String,
    pub description: String,
    pub category: String,
    pub questions: Vec<NewQuestionReq>,
}

pub struct NewQuestionReq {
    pub question_type: String,
    pub title: String,
    pub choices: Vec<NewChoiceReq>
}

pub struct NewChoiceReq {
    pub content: Option<String>,
    pub content_type: String,
    pub title: String,
}
