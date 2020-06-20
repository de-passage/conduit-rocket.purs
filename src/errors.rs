use diesel::result;
use rocket::http::Status;
use rocket::response;
use rocket::response::{status, Responder};
use rocket::Request;
use rocket_contrib::json::{Json, JsonValue};

#[derive(Debug)]
pub enum Error {
    DatabaseError(String, String),
    TokenError(),
    AuthError(),
    Forbidden(),
    Unauthorized(),
}

impl From<result::Error> for Error {
    fn from(err: result::Error) -> Error {
        Error::DatabaseError("".to_owned(), "".to_owned())
    }
}

impl From<Error> for JsonValue {
    fn from(error: Error) -> JsonValue {
        match error {
            Error::DatabaseError(key, value) => json![{
                    key: [value]
            }],
            Error::TokenError() => json![{
                    "token": "encoding failed"
            }],
            Error::AuthError() => json![{
                    "email or password": "is invalid"
            }],
            Error::Unauthorized() => json![{
                "unauthorized": "you need to be logged in to access this resource"
            }],
            Error::Forbidden() => json![{
                "forbidden": "you are not allowed to access this resource"
            }]
        }
    }
}

impl<'r> Responder<'r> for Error {
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        let json = json![{ "errors": JsonValue::from(self) }];
        status::Custom(Status::UnprocessableEntity, Json(json)).respond_to(req)
    }
}
