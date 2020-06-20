use jsonwebtoken::*;
use rocket;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use serde::{Deserialize, Serialize};
use crate::errors;

#[derive(Serialize, Deserialize)]
pub struct AuthData {
    pub id: i32,
    pub username: String,
    pub exp:i32,
}

pub fn encode_token(id: i32, username: &String, secret: &String) -> Option<String> {
    encode(
        &Header::default(),
        &AuthData {
            id,
            username: username.clone(),
            exp: i32::max_value()
        },
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .ok()
}

impl AuthData {
    fn decode(token: &str) -> Option<Self> {
        jsonwebtoken::decode(
            token,
            &DecodingKey::from_secret("secret".as_ref()),
            &Validation::default(),
        )
        .map_err(|e| println!["{}", e])
        .ok()
        .map(|data| data.claims)
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for AuthData {
    type Error = errors::Error;
    fn from_request(request: &'a Request<'r>) -> Outcome<AuthData, Self::Error> {
        match request.headers().get_one("authorization") {
            Some(auth_header) => {
                if auth_header[0..6].to_lowercase() == "token " {
                    println!("Decoding with token {}", &auth_header[6..]);
                    match AuthData::decode(&auth_header[6..]) {
                        Some(data) => Outcome::Success(data),
                        None => Outcome::Failure((Status::Forbidden, errors::Error::Forbidden())),
                    }
                } else {
                    Outcome::Failure((Status::Forbidden, errors::Error::Forbidden()))
                }
            }
            None => Outcome::Failure((Status::Unauthorized, errors::Error::Unauthorized())),
        }
    }
}
