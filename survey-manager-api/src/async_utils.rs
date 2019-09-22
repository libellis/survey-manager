use futures::Future;
use survey_manager_core::app_services::{Payload, decode_payload};
use actix_web::{web, Error as AWError};
use crate::error::{CoreError, TokenError};
use actix_web::error::BlockingError;
use crate::inputs::{CreateSurveyDTO, UpdateSurveyDTO};
use survey_manager_core::app_services::commands::{CreateSurveyCommand, UpdateSurveyCommand};
use std::convert::TryInto;

pub fn decode_payload_async(
    token: String,
) -> impl Future<Item = Payload, Error = AWError> {
    web::block(move || decode_payload(&token) )
        .map_err(|e| {
            match e {
                BlockingError::Error(_) => TokenError::TokenExpired,
                // TODO: Replace with thread blocking error designed for api crate.
                _ => TokenError::TokenExpired,
            }
        })
        .from_err()
}

pub fn try_into_create_cmd_async(
    dto: CreateSurveyDTO,
) -> impl Future<Item = CreateSurveyCommand, Error = AWError> {
    web::block(move || {
        dto.try_into()
    })
        .map_err(|e| {
            match e {
                BlockingError::Error(e) => TokenError::TokenExpired,
                // TODO: Replace with thread blocking error designed for api crate.
                _ => TokenError::TokenExpired,
            }
        })
        .from_err()
}

pub fn try_into_update_cmd_async(
    dto: UpdateSurveyDTO,
) -> impl Future<Item = UpdateSurveyCommand, Error = AWError> {
    web::block(move || {
        dto.try_into()
    })
        .map_err(|e| {
            match e {
                BlockingError::Error(e) => TokenError::TokenExpired,
                // TODO: Replace with thread blocking error designed for api crate.
                _ => TokenError::TokenExpired,
            }
        })
        .from_err()
}
