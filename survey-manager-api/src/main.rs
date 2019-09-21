use actix_web::{web, App, Error as AWError, HttpResponse, HttpServer, Result};
use survey_manager_api::commands::{handle_command_async};
use survey_manager_api::inputs::{CreateSurveyDTO, UpdateSurveyDTO};
use survey_manager_core::app_services::commands::{CreateSurveyCommand, UpdateSurveyCommand};
use survey_manager_core::app_services::token::*;
use futures::{IntoFuture, Future};
use serde_derive::{Serialize, Deserialize};
use dotenv::dotenv;
use uuid::Uuid;
use survey_manager_core::app_services::queries::{FindSurveyQuery, FindSurveysByAuthorQuery};
use survey_manager_api::queries::{handle_queries_async};
use survey_manager_api::extractors::{Token as BearerToken};
use std::convert::TryInto;
use survey_manager_api::error::Error;

// For grabbing a token from get_token endpoint.
#[derive(Serialize)]
struct Token {
    token: String,
}

#[derive(Deserialize)]
pub struct SurveyId {
    id: String,
}

fn create_survey(
    dto: web::Json<CreateSurveyDTO>,
) -> impl Future<Item = HttpResponse, Error = AWError> {
    dto.into_inner().try_into()
        .into_future()
        .from_err()
        .and_then(|cmd: CreateSurveyCommand| {
            handle_command_async(cmd.into())
                .from_err()
                .and_then(|res| Ok(HttpResponse::Ok().json(res)))
        })
}

fn update_survey(
    dto: web::Json<UpdateSurveyDTO>,
) -> impl Future<Item = HttpResponse, Error = AWError> {
    dto.into_inner().try_into()
        .into_future()
        .from_err()
        .and_then(|cmd: UpdateSurveyCommand| {
            handle_command_async(cmd.into())
                .from_err()
                .and_then(|res| Ok(HttpResponse::Ok().json(res)))
        })
}

fn find_survey(
    token: BearerToken,
    params: web::Path<SurveyId>,
) -> impl Future<Item = HttpResponse, Error = AWError> {
    decode_payload(&token.into_inner())
        .map_err(|e| Error::from(e))
        .into_future()
        .from_err()
        .and_then(|Payload{username, ..}| {
            let find_survey_query = FindSurveyQuery {
                id: params.into_inner().id,
                requesting_author: username,
            };

            handle_queries_async(find_survey_query.into())
                .from_err()
                .and_then(|res| {
                    let text = if let Some(s) = res { s } else { "".to_string() };
                    Ok(HttpResponse::Ok()
                        .content_type("application/json")
                        .body(text))
                })
        })
}

fn find_authors_surveys(
    token: BearerToken,
) -> impl Future<Item = HttpResponse, Error = AWError> {
    decode_payload(&token.into_inner())
        .map_err(|e| Error::from(e))
        .into_future()
        .from_err()
        .and_then(|Payload{username, ..}| {
            let find_authors_surveys = FindSurveysByAuthorQuery { author: username, page_config: None };

            handle_queries_async(find_authors_surveys.into())
                .from_err()
                .and_then(|res| {
                    let text = if let Some(s) = res { s } else { "".to_string() };
                    Ok(HttpResponse::Ok()
                        .content_type("application/json")
                        .body(text))
                })
        })
}

fn get_token(
) -> Result<HttpResponse, AWError> {
    let fake_user_id = Uuid::new_v4();
    let token_str = create_token("test_user".to_string(), fake_user_id.to_string());
    let token = Token { token: token_str, };
    Ok(HttpResponse::Ok().json(token))
}

fn main() -> std::io::Result<()> {
    dotenv().ok();

    // Start http server
    HttpServer::new(move || {
        App::new()
            .service(
                web::resource("/survey")
                    .route(web::post().to_async(create_survey))
                    .route(web::patch().to_async(update_survey))
                    .route(web::get().to_async(find_authors_surveys)),
            )
            .service(
                web::resource("/survey/{id}")
                    .route(web::get().to_async(find_survey)),
            )
            .service(
                web::resource("/token")
                    .route(web::get().to(get_token)),
            )
    })
        .bind("127.0.0.1:8080")?
        .run()
}
