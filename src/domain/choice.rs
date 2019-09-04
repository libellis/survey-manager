use domain_patterns::models::Entity;
use uuid::Uuid;
use crate::domain::value_objects::{ContentType, Title};

#[derive(Entity)]
pub struct Choice {
    id: Uuid,
    version: u64,
    question_id: Uuid,
    content: Option<Content>,
    content_type: ContentType,
    title: Title,
}

// Todo: Set this up as a value object once we understand qualifying factors
// for incoming embedded strings that would allow us to type match.
pub enum Content {
    Youtube(String),
    Spotify(String),
    Soundcloud(String),
}
