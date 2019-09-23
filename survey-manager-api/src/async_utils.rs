use futures::Future;
use survey_manager_core::app_services::{Payload, decode_payload};
use actix_web::web;
use crate::error::TokenError;
use crate::inputs::{CreateSurveyDTO, UpdateSurveyDTO};
use survey_manager_core::app_services::commands::{CreateSurveyCommand, UpdateSurveyCommand};
use std::convert::TryInto;

pub fn decode_payload_async(
    token: String,
) -> impl Future<Item = Payload, Error = TokenError> {
    web::block(move || decode_payload(&token).map_err(|_| TokenError::TokenExpired) )
        .from_err()
}

pub fn try_into_create_cmd_async(
    dto: CreateSurveyDTO,
) -> impl Future<Item = CreateSurveyCommand, Error = TokenError> {
    web::block(move || {
        dto.try_into()
    })
        .from_err()
}

pub fn try_into_update_cmd_async(
    dto: UpdateSurveyDTO,
) -> impl Future<Item = UpdateSurveyCommand, Error = TokenError> {
    web::block(move || {
        dto.try_into()
    })
        .from_err()
}
