use uuid::Uuid;
use crate::value_objects::title::Title;
use crate::value_objects::content_type::ContentType;
use crate::dtos::ChoiceDTO;
use std::str::FromStr;
use std::convert::TryFrom;
use crate::survey::Content::Youtube;

#[derive(Entity)]
pub struct Choice {
    pub(super) id: Uuid,
    pub(super) version: u64,
    pub(super) content: Option<Content>,
    pub(super) content_type: ContentType,
    pub(super) title: Title,
}

impl From<ChoiceDTO> for Choice {
    fn from(dto: ChoiceDTO) -> Self {
        let content = if let Some(c) = dto.content {
            // Todo: This is a placehoder.  fix once we figure out streaming content.
            Some(Youtube(c))
        } else {
            None
        };

        Choice {
            id: Uuid::from_str(&dto.id).unwrap().clone(),
            version: dto.version,
            content,
            content_type: ContentType::try_from(dto.content_type).unwrap(),
            title: Title::try_from(dto.title).unwrap(),
        }
    }
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