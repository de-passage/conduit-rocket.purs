use crate::config::Config;
use crate::db::DbResult;
use crate::errors::Error;
use jsonwebtoken::{encode, DecodingKey, EncodingKey, Header, Validation};
use rocket;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::State;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AuthData {
    pub id: i32,
    pub username: String,
    pub exp: i32,
}

pub fn encode_token(id: i32, username: &String, secret: &String) -> DbResult<String> {
    encode(
        &Header::default(),
        &AuthData {
            id,
            username: username.clone(),
            exp: i32::max_value(),
        },
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .map_err(|err| Error::InternalServerError("jwt".to_owned(), err.to_string()))
}

impl AuthData {
    fn decode(token: &str, config: &Config) -> Option<Self> {
        jsonwebtoken::decode(
            token,
            &DecodingKey::from_secret(&config.secret.as_ref()),
            &Validation::default(),
        )
        .map_err(|e| println!["{}", e])
        .ok()
        .map(|data| data.claims)
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for AuthData {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> Outcome<AuthData, Self::Error> {
        let config: State<Config> = request.guard()?;
        match request.headers().get_one("authorization") {
            Some(auth_header) => {
                if auth_header[0..6].to_lowercase() == "token " {
                    match AuthData::decode(&auth_header[6..], config.inner()) {
                        Some(data) => Outcome::Success(data),
                        None => Outcome::Failure((Status::Forbidden, ())),
                    }
                } else {
                    Outcome::Failure((Status::Forbidden, ()))
                }
            }
            None => Outcome::Failure((Status::Unauthorized, ())),
        }
    }
}
