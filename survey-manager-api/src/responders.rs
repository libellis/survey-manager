use actix_web::HttpResponse;
use serde::Serialize;

// IdHateoas allows you to supply an id and the path prefix to generate
// a Hateoas compliant ref about what a user can do next with the resource
// they just created.
#[derive(Serialize)]
pub struct SurveyIdResponder {
    /// Id of the resource.
    pub id: String,

    // Links per HATEOAS convention.
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
            ("removeSurvey", HttpMethod::DELETE),
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

#[derive(Serialize)]
pub struct GetSurveyResponder {
    // This will be the json string delivered by the query handler.  For performance I'm preferring
    // to handle it this way rather than deserializing only to reserialize just so we don't have nested
    // json response.
    pub survey: serde_json::Value,
    // Links per HATEOAS convention.
    pub links: Vec<Link>,
}

impl GetSurveyResponder {
    /// Provide a json string representing the retrieved survey and responder handles the rest.
    pub fn new(survey_json_str: String, id: String) -> GetSurveyResponder {
        let survey_actions = vec![
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

        GetSurveyResponder {
            survey: serde_json::from_str(&survey_json_str).unwrap(),
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
