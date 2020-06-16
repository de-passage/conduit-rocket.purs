#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/users/login")]
fn login() -> &'static str {
    "Hello, world!"
}

#[post("/users")]
fn register() -> &'static str {
    "Hello, world!"
}

#[get("/user")]
fn current_user() -> &'static str {
    "Hello, world!"
}

#[put("/user")]
fn update_current_user() -> &'static str {
    "Hello, world!"
}

#[get("/profiles/<username>")]
fn profile(username: String) -> String {
    format!["Hello, {}", username]
}

#[post("/profiles/<username>/follow")]
fn follow(username: String) -> String {
    format!["Hello, {}", username]
}

#[delete("/profiles/<username>/follow")]
fn unfollow(username: String) -> String {
    format!["Hello, {}", username]
}

#[get("/articles")]
fn articles() -> &'static str {
    "Hello, world!"
}

#[post("/articles")]
fn new_article() -> &'static str {
    "Hello, world!"
}

#[get("/articles/feed")]
fn feed() -> &'static str {
    "Hello, world!"
}

#[get("/articles/<slug>")]
fn article(slug: String) -> String {
    format!["Hello, {}", slug]
}

#[put("/articles/<slug>")]
fn update_article(slug: String) -> String {
    format!["Hello, {}", slug]
}

#[delete("/articles/<slug>")]
fn delete_article(slug: String) -> String {
    format!["Hello, {}", slug]
}

#[get("/articles/<slug>/comments")]
fn comments(slug: String) -> String {
    format!["Hello, {}", slug]
}

#[post("/articles/<slug>/comments")]
fn new_comment(slug: String) -> String {
    format!["Hello, {}", slug]
}

#[delete("/articles/<slug>/comments/<comment_id>")]
fn delete_comment(slug: String, comment_id: u64) -> String {
    format!["Hello, {}/{}", slug, comment_id.to_string()]
}

#[post("/articles/<slug>/favorite")]
fn favorite(slug: String) -> String {
    format!["Hello, {}", slug]
}

#[delete("/articles/<slug>/favorite")]
fn unfavorite(slug: String) -> String {
    format!["Hello, {}", slug]
}

#[get("/tags")]
fn tags() -> &'static str {
    "Hello, tags!"
}

fn main() {
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
        .launch();
}