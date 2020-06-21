#![feature(proc_macro_hygiene, decl_macro)]

mod authentication;
mod db;
mod errors;
mod models;
mod routes;
mod schema;

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel;
extern crate jsonwebtoken;
extern crate scrypt;
extern crate chrono;
extern crate percent_encoding;

use crate::db::DbConnection;
use rocket::Request;
use rocket_cors;
use rocket_cors::{CorsOptions};
use crate::errors::Error;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[catch(401)]
fn unauthorized(_: &Request) -> Error {
    Error::Unauthorized()
}

#[catch(403)]
fn forbidden(_: &Request) -> Error {
    Error::Forbidden()
}

fn cors_options() -> CorsOptions {
    CorsOptions::default()
}

fn main() -> Result<(), rocket_cors::Error> {
    let cors = cors_options().to_cors()?;
    rocket::ignite()
        .mount(
            "/api",
            routes![
                index,
                routes::articles::article,
                routes::articles::articles,
                routes::users::login,
                routes::users::register,
                routes::users::current_user,
                routes::users::update_current_user,
                routes::users::profile,
                routes::users::follow,
                routes::users::unfollow,
                routes::articles::favorite,
                routes::articles::unfavorite,
                routes::comments::comments,
                routes::comments::new_comment,
                routes::comments::delete_comment,
                routes::articles::tags,
                routes::articles::feed,
                routes::articles::new_article,
                routes::articles::update_article,
                routes::articles::delete_article
            ],
        )
        .register(catchers![forbidden, unauthorized])
        .attach(cors)
        .attach(DbConnection::fairing())
        .launch();
    Ok(())
}
