use jsonwebtoken::{encode, decode, Algorithm, Header, Validation};
use chrono::Utc;

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

pub fn decode_payload(token: &str) -> Payload {
    let token_data = decode::<Payload>(token, b"testkey", &Validation::default()).unwrap();
    token_data.claims
}