use crate::authentication::AuthData;
use crate::db;
use crate::db::{DbConnection, DbResult};
use crate::errors::Error;
use crate::models::user::*;
use regex;
use rocket::response;
use rocket::response::Responder;
use rocket::Request;
use rocket_contrib::json::Json;
use serde::Deserialize;

#[derive(Deserialize, Serialize)]
pub struct UserWrapper<U> {
    user: U,
}

type UserResponse = UserWrapper<AuthenticatedUser>;

impl<'r> Responder<'r> for AuthenticatedUser {
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        json![UserResponse { user: self }].respond_to(req)
    }
}

impl<'r> Responder<'r> for Profile {
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        json![{ "profile": self }].respond_to(req)
    }
}

#[post("/users/login", data = "<user>", format = "json")]
pub fn login(
    conn: DbConnection,
    user: Json<UserWrapper<LoginData>>,
) -> DbResult<AuthenticatedUser> {
    db::users::authenticate(
        &conn,
        &user.user.email,
        &user.user.password,
        &"secret".to_owned(),
    )
}

#[post("/users", data = "<data>", format = "json")]
pub fn register(
    conn: DbConnection,
    data: Json<UserWrapper<NewUserData>>,
) -> DbResult<AuthenticatedUser> {
    let user = &data.user;
    let mut errors = json![{}];
    let mut error = false;

    if user.username.is_empty() {
        errors["username"] = json!("is empty").0;
        error = true;
    }

    let regex = email_regex()?;

    if user.email.is_empty() {
        errors["email"] = json!["is empty"].0;
        error = true;
    } else if !regex.is_match(&user.email) {
        errors["email"] = json!["is invalid"].0;
        error = true;
    }

    if user.password.is_empty() {
        errors["password"] = json!["is empty"].0;
        error = true;
    }

    if error {
        Err(Error::ValidationFailed(errors))
    } else {
        db::users::create(
            &conn,
            &user.username,
            &user.email,
            user.password.clone(),
            &"secret".to_owned(),
        )
    }
}

#[get("/user")]
pub fn current_user(conn: DbConnection, auth: AuthData) -> DbResult<AuthenticatedUser> {
    db::users::find_by_id(&conn, auth.id).and_then(|u| u.to_authenticated(&"secret".to_owned()))
}

#[put("/user", data = "<data>", format = "json")]
pub fn update_current_user(
    conn: DbConnection,
    auth: AuthData,
    data: Json<UserWrapper<UserUpdateData>>,
) -> DbResult<AuthenticatedUser> {
    let user = &data.user;
    let mut error = false;
    let mut errors = json![{}];
    match &user.username {
        Some(username) => {
            if username.is_empty() {
                errors["username"] = json!["is empty"].0;
                error = true;
            }
        }
        None => (),
    }
    let regex = email_regex()?;
    match &user.email {
        Some(email) => {
            if email.is_empty() {
                errors["email"] = json!["is empty"].0;
                error = true;
            } else if !regex.is_match(&email) {
                errors["email"] = json!["is invalid"].0;
                error = true;
            }
        }
        None => (),
    }

    match &user.password {
        Some(password) => {
            if password.is_empty() {
                errors["password"] = json!["is empty"].0;
            }
        }
        None => (),
    }

    if error {
        Err(Error::ValidationFailed(errors))
    } else {
        db::users::update(&conn, auth.id, &user, &"secret".to_owned())
    }
}

#[get("/profiles/<username>")]
pub fn profile(conn: DbConnection, username: String, auth: Option<AuthData>) -> DbResult<Profile> {
    db::users::profile(&conn, &username, &auth)
}

#[post("/profiles/<username>/follow")]
pub fn follow(conn: DbConnection, username: String, auth: AuthData) -> DbResult<Profile> {
    db::users::follow(&conn, &username, auth.id)
}

#[delete("/profiles/<username>/follow")]
pub fn unfollow(conn: DbConnection, username: String, auth: AuthData) -> DbResult<Profile> {
    db::users::unfollow(&conn, &username, auth.id)
}

fn email_regex() -> DbResult<regex::Regex> {
    regex::Regex::new(r"^([a-zA-Z0-9_\-\.]+)@([a-zA-Z0-9_\-\.]+)\.([a-zA-Z]{2,5})$")
        .map_err(|err| Error::InternalServerError("email regex".to_owned(), err.to_string()))
}
