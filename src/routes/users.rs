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

#[derive(Deserialize)]
pub struct UserRegistration {
    user: NewUserData,
}

#[post("/users", data = "<user>", format = "json")]
pub fn register(conn: DbConnection, user: Json<UserRegistration>) -> &'static str {
    match db::users::create(
        conn,
        user.user.username.clone(),
        user.user.email.clone(),
        user.user.password.clone(),
    ) {
        Ok(result) => "Ok",
        Err(err) => "Error"
    }
}

#[get("/user")]
pub fn current_user(_conn: DbConnection) -> JsonValue {
    json![User {
        username: "Ho".to_string(),
        bio: None,
        image: None,
        email: "haha@example.com".to_string(),
        id: 0,
        hash: String::default()
    }]
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
