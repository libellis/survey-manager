use crate::dtos::{SurveyDTO, SurveyDTOs};
use crate::app_services::queries::PageConfig;

/// A trait that provides a collection like abstraction over read only database access.
///
/// Generic T is likely a DTO used for pure data transfer to an external caller,
/// whether that's via a REST controller or over gRPC as a proto type etc.
pub trait SurveyDTOReadRepository {
    /// Error type that likely corresponds to an underlying database error.
    type Error: 'static + std::error::Error + std::fmt::Display;

    /// Returns the SurveyDTO corresponding to the supplied key as an owned type.
    ///
    /// # Failure case
    ///
    /// If we fail to communicate with the underlying storage, then an error is returned.
    fn get_survey_for_author(&mut self, id: &String, author: &String) -> Result<Option<SurveyDTO>, Self::Error>;


    /// Returns a `Vec<ListViewSurveyDTO>`, based on the supplied `page_num` and `page_size`.
    /// The page_num should start at 1, but is up to the implementer to design as they see fit.
    /// This is returned as a unique type because the inner `ListViewSurveyDTO` is trimmed down,
    /// and intended for a list view where questions and choices aren't necessary data.
    ///
    /// # Failure case
    ///
    /// If we fail to communicate with the underlying storage, then an error is returned.
    fn get_surveys_by_author(&mut self, author: &String, lower_bound: usize, upper_bound: usize) -> Result<Option<SurveyDTOs>, Self::Error>;
}
