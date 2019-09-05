pub struct NewSurvey {
    pub author: String,
    pub title: String,
    pub description: String,
    pub category: String,
    pub questions: Vec<NewQuestion>,
}

pub struct NewQuestion {
    pub question_type: String,
    pub title: String,
    pub choices: Vec<NewChoice>
}

pub struct NewChoice {
    pub content: Option<String>,
    pub content_type: String,
    pub title: String,
}
