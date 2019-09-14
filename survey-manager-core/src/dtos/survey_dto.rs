use serde::{Serialize, Deserialize};
use std::convert::{From, TryFrom};
use domain_patterns::models::Entity;
use crate::survey::{Choice, Survey, Question};
use uuid::Uuid;
use std::str::FromStr;
use crate::value_objects::{Author, Title, Description, Category, QuestionType, ContentType};

#[derive(Serialize, Deserialize)]
pub struct SurveyDTO {
    pub id: String,
    pub version: u64,
    pub author: String,
    pub title: String,
    pub description: String,
    pub created_on: i64,
    pub category: String,
    pub questions: Vec<QuestionDTO>,
}

#[derive(Serialize, Deserialize)]
pub struct QuestionDTO {
    pub id: String,
    pub version: u64,
    pub question_type: String,
    pub title: String,
    pub choices: Vec<ChoiceDTO>
}

#[derive(Serialize, Deserialize)]
pub struct ChoiceDTO {
    pub id: String,
    pub version: u64,
    pub content: Option<String>,
    pub content_type: String,
    pub title: String,
}

impl From<Survey> for SurveyDTO {
    fn from(s: Survey) -> Self {
        let questions: Vec<QuestionDTO> = s.questions()
            .into_iter()
            .map(|q| QuestionDTO::from(q))
            .collect();
        
        SurveyDTO {
            id: s.id(),
            version: s.version(),
            author: s.author().to_string(),
            title: s.title().to_string(),
            description: s.description().to_string(),
            created_on: s.created_on().clone(),
            category: s.category().to_string(),
            questions,
        }
    }
}

impl From<&Question> for QuestionDTO {
    fn from(q: &Question) -> Self {
        let choices = q.choices()
            .into_iter()
            .map(|c| ChoiceDTO::from(c))
            .collect();

        QuestionDTO {
            id: q.id().to_string(),
            version: q.version(),
            question_type: q.question_type().to_string(),
            title: q.title().to_string(),
            choices,
        }
    }
}

impl From<&Choice> for ChoiceDTO {
    fn from(choice: &Choice) -> Self {
        let content = if let Some(c) = choice.content() {
            Some(c.to_string())
        } else {
            None
        };

        ChoiceDTO {
            id: choice.id().to_string(),
            version: choice.version(),
            content,
            content_type: choice.content_type().to_string(),
            title: choice.title().to_string(),
        }
    }
}

// Same but from reference type, so needs clone.
impl From<&Survey> for SurveyDTO {
    fn from(s: &Survey) -> Self {
        let questions: Vec<QuestionDTO> = s.questions()
            .into_iter()
            .map(|q| QuestionDTO::from(q))
            .collect();

        SurveyDTO {
            id: s.id().to_string(),
            version: s.version(),
            author: s.author().to_string(),
            title: s.title().to_string(),
            description: s.description().to_string(),
            created_on: s.created_on().clone(),
            category: s.category().to_string(),
            questions,
        }
    }
}

//impl Into<Survey> for SurveyDTO {
//    fn into(self) -> Survey {
//        let questions: Vec<Question> = self.questions.into_iter()
//            .map(|q| {
//                q.into()
//            }).collect();
//        // Unwrapping because these should be valid if they are coming from a DTO.
//        Survey {
//            id: Uuid::from_str(self.id.as_ref()).unwrap().clone(),
//            version: self.version,
//            author: Author::try_from(self.author.clone()).unwrap(),
//            title: Title::try_from(self.title.clone()).unwrap(),
//            description: Description::try_from(self.description.clone()).unwrap(),
//            created_on: self.created_on,
//            category: Category::try_from(self.category.clone()).unwrap(),
//            questions,
//        }
//    }
//}
//
//impl Into<Question> for QuestionDTO {
//    fn into(self) -> Question {
//        let choices: Vec<Choice> = self.choices.into_iter()
//            .map(|c| {
//                c.into()
//            }).collect();
//        // Unwrapping because these should be valid if they are coming from a DTO.
//        Question {
//            id: Uuid::from_str(self.id.as_ref()).unwrap().clone(),
//            version: self.version,
//            question_type: QuestionType::try_from(self.question_type.clone()).unwrap(),
//            title: Title::try_from(self.title.clone()).unwrap(),
//            choices,
//        }
//    }
//}
//
//impl Into<Choice> for ChoiceDTO {
//    fn into(self) -> Choice {
//        // Unwrapping because these should be valid if they are coming from a DTO.
//        Choice {
//            id: Uuid::from_str(self.id.as_ref()).unwrap().clone(),
//            version: self.version,
//            // TODO: Fix/update once we actually understand how embedded content works.
//            content: None,
//            content_type: ContentType::try_from(self.content_type.clone()).unwrap(),
//            title: Title::try_from(self.title.clone()).unwrap(),
//        }
//    }
//}
