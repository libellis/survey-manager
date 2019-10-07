use actix_web::{web, App, Error as AWError, HttpResponse, HttpServer, Result};
use survey_manager_api::commands::{handle_command_async};
use survey_manager_api::inputs::{CreateSurveyDTO, UpdateSurveyDTO};
use survey_manager_core::app_services::commands::{CreateSurveyCommand, UpdateSurveyCommand, RemoveSurveyCommand};
use survey_manager_core::app_services::token::*;
use futures::Future;
use serde_derive::{Serialize, Deserialize};
use dotenv::dotenv;
use uuid::Uuid;
use survey_manager_core::app_services::queries::{FindSurveyQuery, FindSurveysByAuthorQuery};
use survey_manager_api::queries::{handle_queries_async};
use survey_manager_api::extractors::{Token as BearerToken};
use survey_manager_api::responders::{SurveyIdResponder, GetSurveyResponder};
use survey_manager_api::async_utils::{decode_payload_async, try_into_create_cmd_async, try_into_update_cmd_async};

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
    try_into_create_cmd_async(dto.into_inner())
        .from_err()
        .and_then(move |cmd: CreateSurveyCommand| {
            handle_command_async(cmd.into())
                .from_err()
                .and_then(move |res| {
                    SurveyIdResponder::new(res).respond()
                })
        })
}

fn update_survey(
    dto: web::Json<UpdateSurveyDTO>,
) -> impl Future<Item = HttpResponse, Error = AWError> {
    try_into_update_cmd_async(dto.into_inner())
        .from_err()
        .and_then(move |cmd: UpdateSurveyCommand| {
            handle_command_async(cmd.into())
                .from_err()
                .and_then(move |res| {
                    SurveyIdResponder::new(res).respond()
                })
        })
}

fn remove_survey(
    token: BearerToken,
    params: web::Path<SurveyId>,
) -> impl Future<Item = HttpResponse, Error = AWError> {
    let id = params.into_inner().id;

    decode_payload_async(token.into_inner())
        .from_err()
        .and_then(move |Payload{username, ..}| {
            let remove_survey_cmd = RemoveSurveyCommand {
                id: id.clone(),
                requesting_author: username,
            };

            handle_command_async(remove_survey_cmd.into())
                .from_err()
                .and_then(move |_| {
                    // TODO: Replace with json response.
                    Ok(HttpResponse::Ok().body("Deleted"))
                })
        })
}

fn find_survey(
    token: BearerToken,
    params: web::Path<SurveyId>,
) -> impl Future<Item = HttpResponse, Error = AWError> {
    let id = params.into_inner().id;

    decode_payload_async(token.into_inner())
        .from_err()
        .and_then(move |Payload{username, ..}| {
            let find_survey_query = FindSurveyQuery {
                id: id.clone(),
                requesting_author: username,
            };

            handle_queries_async(find_survey_query.into())
                .from_err()
                .and_then(move |res| {
                    Ok(GetSurveyResponder::new(res, id).respond())
                })
        })
}

fn find_authors_surveys(
    token: BearerToken,
) -> impl Future<Item = HttpResponse, Error = AWError> {
    decode_payload_async(token.into_inner())
        .from_err()
        .and_then(move |Payload{username, ..}| {
            let find_authors_surveys = FindSurveysByAuthorQuery { author: username, page_config: None };

            handle_queries_async(find_authors_surveys.into())
                .from_err()
                .and_then(move |res| {
                    Ok(HttpResponse::Ok()
                        .content_type("application/json")
                        .body(res))
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

    let addr = match std::env::var("SERVER_HOST") {
        Ok(host) => host,
        Err(_) => "0.0.0.0:8000".to_string(),
    };

    println!("Starting http server: {}", &addr);

    // Start http server
    HttpServer::new(move || {
        App::new()
            .service(
                web::resource("/survey")
                    .route(web::get().to_async(find_authors_surveys))
                    .route(web::post().to_async(create_survey))
                    .route(web::patch().to_async(update_survey)),
            )
            .service(
                web::resource("/survey/{id}")
                    .route(web::get().to_async(find_survey))
                    .route(web::delete().to_async(remove_survey)),
            )
            .service(
                web::resource("/token")
                    .route(web::get().to(get_token)),
            )
    })
        .bind(&addr)?
        .run()
}
