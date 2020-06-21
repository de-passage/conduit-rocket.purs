use diesel::result;
use rocket::http::Status;
use rocket::response;
use rocket::response::{status, Responder};
use rocket::Request;
use rocket_contrib::json::{Json, JsonValue};

#[derive(Debug)]
pub enum Error {
    DatabaseError(String, String),
    InternalServerError(String, String),
    AuthError(),
    Forbidden(),
    Unauthorized(),
}

impl From<result::Error> for Error {
    fn from(err: result::Error) -> Error {
        Error::DatabaseError("database validation".to_owned(), err.to_string())
    }
}

impl<'r> Responder<'r> for Error {
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        let (err, code) = dispatch_error(&self);
        status::Custom(code, Json(json![{ "errors": err }])).respond_to(req)
    }
}

fn dispatch_error(error: &Error) -> (JsonValue, Status) {
    match error {
        Error::DatabaseError(key, value) => (json![{ key: [value] }], Status::UnprocessableEntity),
        Error::InternalServerError(key, value) => {
            (json![{ key: value }], Status::InternalServerError)
        }
        Error::AuthError() => (
            json![{
                    "email or password": "is invalid"
            }],
            Status::UnprocessableEntity,
        ),
        Error::Unauthorized() => (
            json![{
                "unauthorized": "you need to be logged in to access this resource"
            }],
            Status::Unauthorized,
        ),
        Error::Forbidden() => (
            json![{
                "forbidden": "you are not allowed to access this resource"
            }],
            Status::Forbidden,
        ),
    }
}
