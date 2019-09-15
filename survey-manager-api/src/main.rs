use actix_web::{middleware, web, App, Error as AWError, HttpResponse, HttpServer, Result};
use survey_manager_api::commands::{Pool, handle, handle_async};
use survey_manager_api::inputs::{CreateSurveyDTO, UpdateSurveyDTO};
use survey_manager_core::app_services::commands::{SurveyCommands, CreateSurveyCommand, UpdateSurveyCommand};
use survey_manager_core::app_services::token::*;
use futures::{IntoFuture, Future};
use serde_derive::Serialize;
use dotenv::dotenv;
use uuid::Uuid;

#[derive(Serialize)]
struct Token {
    token: String,
}

fn create_survey(
    dto: web::Json<CreateSurveyDTO>,
    pool: web::Data<Pool>,
) -> impl Future<Item = HttpResponse, Error = AWError> {
    let create_survey_command: CreateSurveyCommand = dto.into_inner().into();

    handle_async(&pool, create_survey_command.into())
        .from_err()
        .and_then(|res| Ok(HttpResponse::Ok().json(res)))
}

fn update_survey(
    dto: web::Json<UpdateSurveyDTO>,
    pool: web::Data<Pool>,
) -> impl Future<Item = HttpResponse, Error = AWError> {
    let update_survey_command: UpdateSurveyCommand = dto.into_inner().into();

    handle_async(&pool, update_survey_command.into())
        .from_err()
        .and_then(|res| Ok(HttpResponse::Ok().json(res)))

}

fn get_token(
) -> Result<HttpResponse, AWError> {
    let fake_user_id = Uuid::new_v4();
    let token_str = create_token("test user".to_string(), fake_user_id.to_string());
    let token = Token { token: token_str, };
    Ok(HttpResponse::Ok().json(token))
}

fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = mysql::Pool::new(&database_url).unwrap();

    // Start http server
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .service(
                web::resource("/survey")
                    .route(web::post().to_async(create_survey))
                    .route(web::patch().to_async(update_survey)),
            )
            .service(
                web::resource("/token")
                    .route(web::get().to(get_token)),
            )
    })
        .bind("127.0.0.1:8080")?
        .run()
}
