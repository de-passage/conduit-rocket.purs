use crate::authentication::AuthData;
use crate::db;
use crate::db::{DbConnection, DbResult};
use crate::models::user::*;
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

#[post("/users", data = "<user>", format = "json")]
pub fn register(
    conn: DbConnection,
    user: Json<UserWrapper<NewUserData>>,
) -> DbResult<AuthenticatedUser> {
    db::users::create(
        &conn,
        &user.user.username,
        &user.user.email,
        user.user.password.clone(),
        &"secret".to_owned(),
    )
}

#[get("/user")]
pub fn current_user(conn: DbConnection, auth: AuthData) -> DbResult<AuthenticatedUser> {
    db::users::find_by_id(&conn, auth.id).and_then(|u| u.to_authenticated(&"secret".to_owned()))
}

#[put("/user", data = "<user>", format = "json")]
pub fn update_current_user(
    conn: DbConnection,
    auth: AuthData,
    user: Json<UserWrapper<UserUpdateData>>,
) -> DbResult<AuthenticatedUser> {
    db::users::update(&conn, auth.id, &user.user, &"secret".to_owned())
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
