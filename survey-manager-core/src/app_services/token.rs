use jsonwebtoken::{encode, decode, Header, Validation};
use chrono::Utc;
use crate::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct Payload {
    pub username: String,
    pub user_id: String,
    pub exp: i64,
}

pub fn create_token(username: String, user_id: String) -> String {

    // set timeout on JWT to 30 minutes (1800 seconds) after you get it
    let new_payload = Payload {
        username,
        user_id,
        exp: Utc::now().timestamp() + 1800,
    };

    encode(&Header::default(), &new_payload, "testkey".as_ref()).unwrap()
}

pub fn decode_payload(token: &str) -> Result<Payload, Error> {
    let token_data = decode::<Payload>(token, b"testkey", &Validation::default())
        .map_err(|_| Error::NotAuthorized)?;
    Ok(token_data.claims)
}