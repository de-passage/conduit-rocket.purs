use diesel::result;
use rocket_contrib::json::JsonValue;

pub enum Error {
    DatabaseError(String, String),
    TokenError(),
    AuthError()
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
                "errors": {
                    key: [value]
                }
            }],
            Error::TokenError() => json![{
                "errors": {
                    "token": "encoding failed"
                }
            }],
            Error::AuthError() => json![{
                "errors": {
                    "email or password": "is invalid"
                }
            }]
        }
    }
}