use uuid::Uuid;
use crate::value_objects::title::Title;
use crate::value_objects::content_type::ContentType;

#[derive(Entity)]
pub struct Choice {
    pub(super) id: Uuid,
    pub(super) version: u64,
    pub(super) content: Option<Content>,
    pub(super) content_type: ContentType,
    pub(super) title: Title,
}

// Todo: Set this up as a value object once we understand qualifying factors
// for incoming embedded strings that would allow us to type match.
#[derive(Clone)]
pub enum Content {
    Youtube(String),
    Spotify(String),
    Soundcloud(String),
}

impl std::fmt::Display for Content {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let value = match self {
            Content::Youtube(value) => value,
            Content::Spotify(value) => value,
            Content::Soundcloud(value) => value,
        };

        write!(f, "{}", value)
    }
}