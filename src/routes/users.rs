use crate::db;
use crate::db::{DbConnection, DbResult};
use crate::models::user::*;
use rocket::response;
use rocket::response::Responder;
use rocket::Request;
use rocket_contrib::json::{Json, JsonValue};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct NewUserData {
    username: String,
    email: String,
    password: String,
}

#[derive(Deserialize)]
pub struct LoginData {
    email: String,
    password: String,
}

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
pub fn login(conn: DbConnection, user: Json<UserWrapper<LoginData>>) -> DbResult<AuthenticatedUser> {
    db::users::authenticate(&conn, &user.user.email, &user.user.password)
}

#[post("/users", data = "<user>", format = "json")]
pub fn register(conn: DbConnection, user: Json<UserWrapper<NewUserData>>) -> DbResult<AuthenticatedUser> {
    db::users::create(
        &conn,
        &user.user.username,
        &user.user.email,
        &user.user.password)
}

#[get("/user")]
pub fn current_user(_conn: DbConnection) -> JsonValue {
    json![{}]
}

#[put("/user")]
pub fn update_current_user(_conn: DbConnection) -> &'static str {
    "Hello, world!"
}

#[get("/profiles/<username>")]
pub fn profile(conn: DbConnection, username: String) -> DbResult<Profile> {
    db::users::profile(&conn, &username, &None)
}

#[post("/profiles/<username>/follow")]
pub fn follow(_conn: DbConnection, username: String) -> String {
    format!["Hello, {}", username]
}

#[delete("/profiles/<username>/follow")]
pub fn unfollow(_conn: DbConnection, username: String) -> String {
    format!["Hello, {}", username]
}
