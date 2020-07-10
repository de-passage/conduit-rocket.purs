#![feature(proc_macro_hygiene, decl_macro)]

mod authentication;
mod config;
mod db;
mod errors;
mod format;
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

use crate::config::{configure_rocket, Config};
use crate::db::DbConnection;
use crate::errors::Error;
use dotenv::dotenv;
use rocket::Request;
use rocket_cors;
use rocket_cors::CorsOptions;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[catch(401)]
fn unauthorized(_: &Request) -> Error {
    Error::Unauthorized
}

#[catch(403)]
fn forbidden(_: &Request) -> Error {
    Error::Forbidden
}

fn cors_options() -> CorsOptions {
    CorsOptions::default()
}

fn main() -> Result<(), String> {
    if cfg!(debug_assertions) || cfg!(test) {
        dotenv().ok();
    }

    let cors = cors_options().to_cors().map_err(|err| err.to_string())?;
    let config = Config::from_env()?;
    rocket::custom(configure_rocket()?)
        .manage(config)
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
