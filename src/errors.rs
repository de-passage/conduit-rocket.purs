use diesel::result;
use rocket::http::Status;
use rocket::response;
use rocket::response::{status, Responder};
use rocket::Request;
use rocket_contrib::json::{Json, JsonValue};

#[derive(Debug)]
pub enum Error {
    DatabaseError(result::Error),
    ValidationFailed(JsonValue),
    InternalServerError(String, String),
    AuthError,
    Forbidden,
    Unauthorized,
}

impl From<result::Error> for Error {
    fn from(err: result::Error) -> Error {
        Error::DatabaseError(err)
    }
}

impl<'r> Responder<'r> for Error {
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        let (err, code) = dispatch_error(self);
        status::Custom(code, Json(json![{ "errors": err }])).respond_to(req)
    }
}

use diesel::result::Error::*;

fn dispatch_error(error: Error) -> (JsonValue, Status) {
    match error {
        Error::DatabaseError(err) => match err {
            InvalidCString(null_err) => (
                json![{
                    "ill-formed query": format!["nul byte at position {}", null_err.nul_position()]
                }],
                Status::InternalServerError,
            ),
            DatabaseError(kind, info) => (make_error_msg(kind, info), Status::UnprocessableEntity),
            NotFound => (
                json![{"not found": "requested resource not found"}],
                Status::NotFound,
            ),
            QueryBuilderError(err) => (
                json![{"query builder": err.to_string()}],
                Status::InternalServerError,
            ),
            DeserializationError(err) => (
                json![{"deserialization": err.to_string()}],
                Status::InternalServerError,
            ),
            SerializationError(err) => (
                json![{"deserialization": err.to_string() }],
                Status::InternalServerError,
            ),
            RollbackTransaction => (
                json![{"rollback": "server-initiated rollback"}],
                Status::InternalServerError,
            ),
            AlreadyInTransaction => (
                json![{"already in transaction": "invalid operation within a transaction"}],
                Status::InternalServerError,
            ),
            _ => (
                json![{"database": "responded with unknown error code"}],
                Status::InternalServerError,
            ),
        },
        Error::InternalServerError(key, value) => {
            (json![{ key: value }], Status::InternalServerError)
        }
        Error::AuthError => (
            json![{
                    "email or password": "is invalid"
            }],
            Status::UnprocessableEntity,
        ),
        Error::Unauthorized => (
            json![{
                "unauthorized": "you need to be logged in to access this resource"
            }],
            Status::Unauthorized,
        ),
        Error::Forbidden => (
            json![{
                "forbidden": "you are not allowed to access this resource"
            }],
            Status::Forbidden,
        ),
        Error::ValidationFailed(value) => (value, Status::UnprocessableEntity),
    }
}

use diesel::result::{DatabaseErrorInformation, DatabaseErrorKind};
fn make_error_msg(
    kind: DatabaseErrorKind,
    info: Box<dyn DatabaseErrorInformation + Send + Sync>,
) -> JsonValue {
    let table = info.table_name().unwrap_or("<unknown table>");
    let col = info.column_name().unwrap_or("<unknown column>");
    let key = format!["{} {}", table, col];
    let value = match kind {
        DatabaseErrorKind::UniqueViolation => "already exists",
        DatabaseErrorKind::ForeignKeyViolation => "foreign key doesn't exist",
        _ => "unhandled error occured",
    };
    json![{ key: [value] }]
}
