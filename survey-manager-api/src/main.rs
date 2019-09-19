use actix_web::{middleware, web, App, Error as AWError, HttpResponse, HttpServer, Result, HttpRequest, Either, http};
use survey_manager_api::commands::{Pool, handle, handle_command_async};
use survey_manager_api::inputs::{CreateSurveyDTO, UpdateSurveyDTO};
use survey_manager_core::app_services::commands::{SurveyCommands, CreateSurveyCommand, UpdateSurveyCommand};
use survey_manager_core::app_services::token::*;
use futures::{IntoFuture, Future};
use serde_derive::{Serialize, Deserialize};
use dotenv::dotenv;
use uuid::Uuid;
use survey_manager_core::app_services::queries::{FindSurveyQuery, FindSurveysByAuthorQuery};
use survey_manager_api::queries::{handle_queries_async, handle_queries_async_no_cache};
use actix_web::error::ErrorUnauthorized;
use futures::future::err;
use survey_manager_api::extractors::{Token as BearerToken, token_from_req};
use survey_manager_infra::utils::redis_pool::{Pool as RedisPool, create_pool};

use log::debug;
use r2d2_redis::RedisConnectionManager;

const MISSING_TOKEN_STR: &'static str = "You must supply a JWT as a bearer token in the auth headers to access that resource.";

// For grabbing a token from get_token endpoint.
#[derive(Serialize)]
struct Token {
    token: String,
}

#[derive(Deserialize)]
pub struct SurveyId {
    id: String,
}

#[derive(Deserialize)]
pub struct Author {
    author: String,
}

fn create_survey(
    dto: web::Json<CreateSurveyDTO>,
) -> impl Future<Item = HttpResponse, Error = AWError> {
    let create_survey_command: CreateSurveyCommand = dto.into_inner().into();

    handle_command_async(create_survey_command.into())
        .from_err()
        .and_then(|res| Ok(HttpResponse::Ok().json(res)))
}

fn update_survey(
    dto: web::Json<UpdateSurveyDTO>,
) -> impl Future<Item = HttpResponse, Error = AWError> {
    let update_survey_command: UpdateSurveyCommand = dto.into_inner().into();

    handle_command_async(update_survey_command.into())
        .from_err()
        .and_then(|res| Ok(HttpResponse::Ok().json(res)))
}

fn find_survey(
    //token: web::Data<BearerToken>,
    req: HttpRequest,
    params: web::Path<SurveyId>,
) -> impl Future<Item = HttpResponse, Error = AWError> {
    let token = token_from_req(&req).unwrap();
    let Payload{username, ..} = decode_payload(&token);
    // let Payload{username, ..} = decode_payload(&token.into_inner());

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
}

fn find_survey_uncached(
    //token: web::Data<BearerToken>,
    req: HttpRequest,
    params: web::Path<SurveyId>,
) -> impl Future<Item = HttpResponse, Error = AWError> {
    let token = token_from_req(&req).unwrap();
    let Payload{username, ..} = decode_payload(&token);
    // let Payload{username, ..} = decode_payload(&token.into_inner());

    let find_survey_query = FindSurveyQuery {
        id: params.into_inner().id,
        requesting_author: username,
    };


    handle_queries_async_no_cache(find_survey_query.into())
        .from_err()
        .and_then(|res| {
            let text = if let Some(s) = res { s } else { "".to_string() };
            Ok(HttpResponse::Ok()
                .content_type("application/json")
                .body(text))
        })
}

fn find_authors_surveys(
    //token: web::Data<BearerToken>,
    req: HttpRequest,
) -> impl Future<Item = HttpResponse, Error = AWError> {
    let token = token_from_req(&req).unwrap();
    let Payload{username, ..} = decode_payload(&token);
//    let Payload{username, ..} = decode_payload(&token.into_inner());

    let find_authors_surveys = FindSurveysByAuthorQuery { author: username, page_config: None };

    handle_queries_async(find_authors_surveys.into())
        .from_err()
        .and_then(|res| {
            let text = if let Some(s) = res { s } else { "".to_string() };
            Ok(HttpResponse::Ok()
                .content_type("application/json")
                .body(text))
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
                web::resource("/survey_uncached/{id}")
                    .route(web::get().to_async(find_survey_uncached)),
            )
            .service(
                web::resource("/token")
                    .route(web::get().to(get_token)),
            )
    })
        .bind("127.0.0.1:8080")?
        .run()
}
