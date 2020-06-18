use jsonwebtoken::*;
use serde::{Deserialize, Serialize};
use rocket::request::{FromRequest, Request, Outcome};
use rocket;
use rocket::http::Status;

#[derive(Serialize, Deserialize)]
pub struct AuthData {
    id: u32,
    username: String,
}

impl AuthData {
    pub fn encode(self: &Self) -> Option<String> {
        encode(
            &Header::default(),
            self,
            &EncodingKey::from_secret("secret".as_ref()),
        )
        .ok()
    }

    fn decode(token: &str) -> Option<Self> {
        jsonwebtoken::decode(
            token,
            &DecodingKey::from_secret("secret".as_ref()),
            &Validation::default(),
        )
        .ok()
        .map(|data| data.claims)
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for AuthData {
    type Error = ();
    fn from_request(request:&'a Request<'r>) -> Outcome<AuthData, Self::Error> {
        match request.headers().get_one("authorization") {
            Some(auth_header) =>  {
                if auth_header.starts_with("TOKEN ") {
                    match AuthData::decode(&auth_header[6..]) {
                        Some(data) => Outcome::Success(data),
                        None => Outcome::Failure((Status::Forbidden, ()))
                    }
                }
                else {
                    Outcome::Failure((Status::Forbidden, ()))
                }
            },
            None => Outcome::Failure((Status::Unauthorized, ()))
        }
    }
}
