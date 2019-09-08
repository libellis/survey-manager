pub struct NewSurveyData {
    pub author: String,
    pub title: String,
    pub description: String,
    pub category: String,
    pub questions: Vec<NewQuestionData>,
}

pub struct NewQuestionData {
    pub question_type: String,
    pub title: String,
    pub choices: Vec<NewChoiceData>
}

pub struct NewChoiceData {
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
