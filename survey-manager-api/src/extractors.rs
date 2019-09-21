use actix_web::{FromRequest, HttpRequest, Error, http::header};
use std::sync::Arc;
use futures::Future;
use crate::error::TokenError;
use actix_web::dev::Payload;

pub struct Token {
    inner: String
}

impl Token {
    /// Deconstruct to an inner value
    pub fn into_inner(self) -> String {
        self.inner
    }
}

impl AsRef<String> for Token {
    fn as_ref(&self) -> &String {
        &self.inner
    }
}

impl std::ops::Deref for Token {
    type Target = String;

    fn deref(&self) -> &String {
        &self.inner
    }
}

impl std::ops::DerefMut for Token {
    fn deref_mut(&mut self) -> &mut String {
        &mut self.inner
    }
}

impl From<String> for Token {
    fn from(inner: String) -> Self {
        Token { inner }
    }
}

impl std::fmt::Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}

impl FromRequest for Token {
    type Error = Error;
    type Future = Result<Self, Error>;
    type Config = TokenConfig;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let error_handler = req
            .app_data::<Self::Config>()
            .map(|c| c.ehandler.clone())
            .unwrap_or(None);

        match token_from_req(req) {
            Some(inner) => Ok(Token { inner }),
            None => {
                let e = TokenError::MissingBearer;
                if let Some(error_handler) = error_handler {
                    Err((error_handler)(e, req))
                } else {
                    Err(e.into())
                }
            }
        }
    }
}

#[derive(Clone)]
pub struct TokenConfig {
    ehandler: Option<Arc<dyn Fn(TokenError, &HttpRequest) -> Error + Send + Sync>>,
}

impl TokenConfig {
    /// Set custom error handler
    pub fn error_handler<F>(mut self, f: F) -> Self
        where
            F: Fn(TokenError, &HttpRequest) -> Error + Send + Sync + 'static,
    {
        self.ehandler = Some(Arc::new(f));
        self
    }
}

impl Default for TokenConfig {
    fn default() -> Self {
        TokenConfig { ehandler: None }
    }
}

pub fn token_from_req(req: &HttpRequest) -> Option<String> {
    let headers =
        if let Some(h) =  req
            .headers()
            .get(header::AUTHORIZATION) {
            h
        } else {
            return None;
        };

    if let Ok(t) = headers.to_str() {
        let mut token_iterator = t.split_whitespace();
        if let Some(identifier) = token_iterator.next() {
            if identifier != "Bearer" { return None; }

            if let Some(token) = token_iterator.next() {
                return Some(token.to_string())
            }
        }
    }

    None
}
