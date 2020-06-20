use crate::db;
use crate::db::DbConnection;
use crate::models::user::*;
use rocket_contrib::json::{Json, JsonValue};
use serde::Deserialize;

#[post("/users/login")]
pub fn login(_conn: DbConnection) -> &'static str {
    "Hello, world!"
}

#[derive(Deserialize)]
pub struct NewUserData {
    username: String,
    email: String,
    password: String,
}

#[derive(Deserialize, Serialize)]
pub struct UserWrapper<U> {
    user: U,
}

type UserResponse = UserWrapper<AuthenticatedUser>;

#[post("/users", data = "<user>", format = "json")]
pub fn register(conn: DbConnection, user: Json<UserWrapper<NewUserData>>) -> JsonValue {
    match db::users::create(
        conn,
        &user.user.username,
        &user.user.email,
        &user.user.password,
    ) {
        Ok(result) => json![UserResponse{ user: result }],
        Err(err) => JsonValue::from(err)
    }
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
pub fn profile(_conn: DbConnection, username: String) -> String {
    format!["Hello, {}", username]
}

#[post("/profiles/<username>/follow")]
pub fn follow(_conn: DbConnection, username: String) -> String {
    format!["Hello, {}", username]
}

#[delete("/profiles/<username>/follow")]
pub fn unfollow(_conn: DbConnection, username: String) -> String {
    format!["Hello, {}", username]
}
