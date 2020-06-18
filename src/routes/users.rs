use crate::db::DbConnection;
use crate::models::user::*;
use rocket_contrib::json::JsonValue;

#[post("/users/login")]
pub fn login(_conn: DbConnection) -> &'static str {
    "Hello, world!"
}

#[post("/users")]
pub fn register(_conn: DbConnection) -> &'static str {
    "Hello, world!"
}

#[get("/user")]
pub fn current_user(_conn: DbConnection) -> JsonValue {
    json![User {
        username: Username("Ho".to_string()),
        bio: None,
        image: None,
        email: "haha@example.com".to_string(),
        id: 0,
        token: String::default()
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