use actix_web::{Error as AWError, HttpRequest, HttpResponse, Responder};
use serde::Serialize;
use futures::{Future, IntoFuture};

// IdHateoas allows you to supply an id and the path prefix to generate
// a Hateoas compliant ref about what a user can do next with the resource
// they just created.
#[derive(Serialize)]
pub struct SurveyIdResponder {
    /// Id of the resource.
    pub id: String,

    /// Resource path so we can create relative paths.
    pub links: Vec<Link>,
}

#[derive(Serialize)]
pub struct Link {
    pub rel: &'static str,
    pub href: String,
    pub method: HttpMethod,
}

#[derive(Serialize)]
pub enum HttpMethod {
    GET,
    PUT,
    PATCH,
    POST,
    DELETE,
}

impl SurveyIdResponder {
    pub fn new(id: String) -> SurveyIdResponder {
        let survey_actions = vec![
            ("getSurvey", HttpMethod::GET),
            ("updateSurvey", HttpMethod::PATCH),
            ("deleteSurvey", HttpMethod::DELETE),
        ];

        let links = survey_actions.into_iter().map(|(action, method)| {
            Link {
                rel: action,
                href: format!("/survey/{}", id),
                method,
            }
        }).collect();

        SurveyIdResponder {
            id,
            links,
        }
    }

    // Adding this method because I can't figure out how to create an async `Responder`
    pub fn respond(&self) -> HttpResponse {
        let body = serde_json::to_string(&self).unwrap();

        // Create response and set content type
        HttpResponse::Ok()
            .content_type("application/json")
            .body(body)
    }
}
