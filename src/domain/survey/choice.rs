use domain_patterns::models::Entity;
use uuid::Uuid;
use crate::domain::value_objects::title::Title;
use crate::domain::value_objects::content_type::ContentType;
use std::error::Error;
use std::convert::TryFrom;

#[derive(Entity)]
pub struct Choice {
    pub(super) id: Uuid,
    pub(super) version: u64,
    pub(super) content: Option<Content>,
    pub(super) content_type: ContentType,
    pub(super) title: Title,
}

// COMMENTING OUT BECAUSE ONLY AGGREGATE ROOT SHOULD HAVE A CONSTRUCTOR
//impl Choice {
//    // TODO: Improve error so it's not ambiguous boxed type.
//    // TODO: Add ability to pass content in once we have value object setup.
//    fn new(question_id: Uuid, content_type: String, title: String) -> Result<Choice, Box<dyn Error>> {
//        Ok(Choice {
//            id: Uuid::new_v4(),
//            version: 0,
//            question_id,
//            content: None,
//            content_type: ContentType::try_from(content_type)?,
//            title: Title::try_from(title)?,
//        })
//    }
//}

// Todo: Set this up as a value object once we understand qualifying factors
// for incoming embedded strings that would allow us to type match.
pub enum Content {
    Youtube(String),
    Spotify(String),
    Soundcloud(String),
}
