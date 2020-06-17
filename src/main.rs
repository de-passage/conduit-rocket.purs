#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
use rocket_cors;
use rocket_cors::{CorsOptions, Error};
use rocket_contrib::databases::diesel;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/users/login")]
fn login(_conn: DbConnection) -> &'static str {
    "Hello, world!"
}

#[post("/users")]
fn register(_conn: DbConnection) -> &'static str {
    "Hello, world!"
}

#[get("/user")]
fn current_user(_conn: DbConnection) -> &'static str {
    "Hello, world!"
}

#[put("/user")]
fn update_current_user(_conn: DbConnection) -> &'static str {
    "Hello, world!"
}

#[get("/profiles/<username>")]
fn profile(_conn: DbConnection, username: String) -> String {
    format!["Hello, {}", username]
}

#[post("/profiles/<username>/follow")]
fn follow(_conn: DbConnection, username: String) -> String {
    format!["Hello, {}", username]
}

#[delete("/profiles/<username>/follow")]
fn unfollow(_conn: DbConnection, username: String) -> String {
    format!["Hello, {}", username]
}

#[get("/articles?<tag>&<author>&<offset>&<limit>&<favorited>")]
fn articles(
    _conn: DbConnection,
    tag: Option<String>,
    author: Option<String>,
    offset: Option<u32>,
    limit: Option<u32>,
    favorited: Option<bool>,
) -> String {
    let params = vec![
        tag,
        author,
        offset.map(|x| x.to_string()),
        limit.map(|x| x.to_string()),
        favorited.map(|x| x.to_string()),
    ]
    .into_iter()
    .flat_map(|o| o.into_iter())
    .collect::<Vec<_>>()
    .join(", ");
    format!("Hello, world!?{}", params)
}

#[post("/articles")]
fn new_article(_conn: DbConnection) -> &'static str {
    "Hello, world!"
}

#[get("/articles/feed?<limit>&<offset>")]
fn feed(_conn: DbConnection, limit: Option<u32>, offset: Option<u32>) -> String {
    let params = vec![limit, offset]
        .into_iter()
        .flat_map(|o| o.into_iter().map(|x| x.to_string()))
        .collect::<Vec<_>>()
        .join(", ");
    format!("Hello, world?{}", params)
}

#[get("/articles/<slug>")]
fn article(_conn: DbConnection, slug: String) -> String {
    format!["Hello, {}", slug]
}

#[put("/articles/<slug>")]
fn update_article(_conn: DbConnection, slug: String) -> String {
    format!["Hello, {}", slug]
}

#[delete("/articles/<slug>")]
fn delete_article(_conn: DbConnection, slug: String) -> String {
    format!["Hello, {}", slug]
}

#[get("/articles/<slug>/comments")]
fn comments(_conn: DbConnection, slug: String) -> String {
    format!["Hello, {}", slug]
}

#[post("/articles/<slug>/comments")]
fn new_comment(_conn: DbConnection, slug: String) -> String {
    format!["Hello, {}", slug]
}

#[delete("/articles/<slug>/comments/<comment_id>")]
fn delete_comment(_conn: DbConnection, slug: String, comment_id: u64) -> String {
    format!["Hello, {}/{}", slug, comment_id.to_string()]
}

#[post("/articles/<slug>/favorite")]
fn favorite(_conn: DbConnection, slug: String) -> String {
    format!["Hello, {}", slug]
}

#[delete("/articles/<slug>/favorite")]
fn unfavorite(_conn: DbConnection, slug: String) -> String {
    format!["Hello, {}", slug]
}

#[get("/tags")]
fn tags(_conn: DbConnection) -> &'static str {
    "Hello, tags!"
    
}

fn cors_options() -> CorsOptions {
    CorsOptions::default()
}

#[database("postgres")]
struct DbConnection(diesel::PgConnection);

fn main() -> Result<(), Error> {
    let cors = cors_options().to_cors()?;
    rocket::ignite()
        .mount(
            "/api",
            routes![
                index,
                article,
                articles,
                login,
                register,
                current_user,
                update_current_user,
                profile,
                follow,
                unfollow,
                favorite,
                unfavorite,
                comments,
                new_comment,
                delete_comment,
                tags,
                feed,
                new_article,
                update_article,
                delete_article
            ],
        )
        .attach(cors)
        .attach(DbConnection::fairing())
        .launch();
    Ok(())
}
