use actix_web::{http::header, HttpRequest};

pub fn token_from_req(req: HttpRequest) -> Option<String> {
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

            // safe to unwrap because we've confirmed first option is success.
            if let Some(token) = token_iterator.next() {
                return Some(token.to_string())
            }
        }
    }

    None
}