use actix_web::{middleware, web, App, Error as AWError, HttpResponse, HttpServer, Result, HttpRequest, Either, http};
use survey_manager_api::commands::{handle, handle_command_async};
use survey_manager_api::inputs::{CreateSurveyDTO, UpdateSurveyDTO};
use survey_manager_core::app_services::commands::{SurveyCommands, CreateSurveyCommand, UpdateSurveyCommand};
use survey_manager_core::app_services::token::*;
use futures::{IntoFuture, Future};
use serde_derive::{Serialize, Deserialize};
use dotenv::dotenv;
use uuid::Uuid;
use survey_manager_core::app_services::queries::{FindSurveyQuery, FindSurveysByAuthorQuery};
use survey_manager_api::queries::{handle_queries_async, handle_queries_async_no_cache};
use survey_manager_api::extractors::{Token as BearerToken, token_from_req};
use survey_manager_api::generate;
use survey_manager_infra::mysql_repos::{MysqlSurveyDTOsRepository, MysqlSurveyWriteRepository};
use survey_manager_infra::cache_repo_decorators::RedisCacheRepository;
use survey_manager_core::app_services::repository_contracts::SurveyDTOReadRepository;
use survey_manager_api::generate::QueryHandler;
use domain_patterns::query::HandlesQuery;
use std::sync::{Mutex, Arc};
use domain_patterns::collections::Repository;

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

fn pure_test_cached() -> impl Future<Item = HttpResponse, Error = AWError> {
    let author = "test_user".to_string();
    let s_id = "9324f63d-545b-47fb-be7d-f560bb7476ef".to_string();
    web::block(move || {
        let mut mysql_repo = MysqlSurveyDTOsRepository::new();
        let mut cached_repo = RedisCacheRepository::new(mysql_repo);
        cached_repo.get_survey_for_author(&s_id, &author)
            .map_err(|e| {
                let error: SMError = RepoFailure {source: Box::new(e)}.into();
                Error::from(error)
            })
    }).from_err().and_then(|_|{
        Ok(HttpResponse::Ok().body("test"))
    })
}

use survey_manager_api::error::Error;
use survey_manager_core::Error as SMError;

fn pure_test_uncached(
) -> impl Future<Item = HttpResponse, Error = AWError> {
    let author = "test_user".to_string();
    let s_id = "9324f63d-545b-47fb-be7d-f560bb7476ef".to_string();
    web::block(move || {
        let mut mysql_repo = MysqlSurveyDTOsRepository::new();
        mysql_repo.get_survey_for_author(&s_id, &author)
            .map_err(|e| {
                let error: SMError = RepoFailure {source: Box::new(e)}.into();
                Error::from(error)
            })
    }).from_err().and_then(|_|{
        Ok(HttpResponse::Ok().body("test"))
    })
}

use domain_patterns::command::Handles;
use actix_web::middleware::Logger;
use survey_manager_core::errors::Error::RepoFailure;

fn write_test() -> Result<HttpResponse, AWError> {
    let mut handler = generate::command_handler();
    let mut cmd = UpdateSurveyCommand {
        id: "9324f63d-545b-47fb-be7d-f560bb7476ef".to_string(),
        author: "test_user".to_string(),
        title: Some("New test title".to_string()),
        description: None,
        category: None,
        questions: None
    };
    let s_id = handler.handle(cmd).unwrap();

    Ok(HttpResponse::Ok().body("test"))
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
            .service(
                web::resource("/pure-test-cached")
                    .route(web::get().to_async(pure_test_cached)),
            )
            .service(
                web::resource("/pure-test")
                    .route(web::get().to_async(pure_test_uncached)),
            )
            .service(
                web::resource("/write-test")
                    .route(web::get().to(write_test)),
            )
    })
        .bind("127.0.0.1:8080")?
        .run()
}
