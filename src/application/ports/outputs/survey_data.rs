use serde::Serialize;
use std::convert::Into;
use crate::domain::survey::{Choice, Survey};

#[derive(Serialize)]
pub struct SurveyCreated {
    pub id: String,
    pub version: u64,
    pub author: String,
    pub title: String,
    pub description: String,
    // TODO: Change into nice timestamp.
    pub created_on: i64,
    pub category: String,
    pub questions: Vec<QuestionCreated>,
}

#[derive(Serialize)]
pub struct QuestionCreated {
    pub id: String,
    pub version: u64,
    pub question_type: String,
    pub title: String,
    pub choices: Vec<ChoiceCreated>
}

#[derive(Serialize)]
pub struct ChoiceCreated {
    pub id: String,
    pub version: u64,
    pub content: Option<String>,
    pub content_type: String,
    pub title: String,
}

//impl Into<SurveyCreated> for Survey {
//    fn into(self) -> SurveyCreated {
//        let questions: Vec<NewQuestion> = self.questions
//            .into_iter()
//            .map(|q| {
//                q.into()
//            }).collect();
//
//        // TODO: Change to actually parse author from token once we implement token logic.
//        let author = self.token;
//        NewSurvey {
//            author,
//            title: self.title,
//            description: self.description,
//            category: self.category,
//            questions,
//        }
//    }
//}
//
//impl Into<NewQuestion> for NewQuestionData {
//    fn into(self) -> NewQuestion {
//        let choices: Vec<NewChoice> = self.choices
//            .into_iter()
//            .map(|c| {
//                c.into()
//            }).collect();
//
//        NewQuestion {
//            question_type: self.question_type,
//            title: self.title,
//            choices,
//        }
//    }
//}

impl From<Choice> for ChoiceCreated {
    fn from(choice: Choice) -> Self {
        ChoiceCreated {
            id: choice.id(),
            version: choice.version(),
            content: choice.content(),
            content_type: choice.content_type(),
            title: choice.title(),
        }
    }
}
