use crate::db::DbConnection;
use crate::authentication::AuthData;
use crate::models;
use crate::db;

#[get("/articles?<tag>&<author>&<offset>&<limit>&<favorited>")]
pub fn articles(
    conn: DbConnection,
    option: AuthData,
    tag: Option<String>,
    author: Option<String>,
    offset: Option<u32>,
    limit: Option<u32>,
    favorited: Option<bool>,
) {
}

#[post("/articles")]
pub fn new_article(_conn: DbConnection) -> &'static str {
    "Hello, world!"
}

#[get("/articles/feed?<limit>&<offset>")]
pub fn feed(_conn: DbConnection, limit: Option<u32>, offset: Option<u32>) -> String {
    let params = vec![limit, offset]
        .into_iter()
        .flat_map(|o| o.into_iter().map(|x| x.to_string()))
        .collect::<Vec<_>>()
        .join(", ");
    format!("Hello, world?{}", params)
}

#[get("/articles/<slug>")]
pub fn article(_conn: DbConnection, slug: String) -> String {
    format!["Hello, {}", slug]
}

#[put("/articles/<slug>")]
pub fn update_article(_conn: DbConnection, slug: String) -> String {
    format!["Hello, {}", slug]
}

#[delete("/articles/<slug>")]
pub fn delete_article(_conn: DbConnection, slug: String) -> String {
    format!["Hello, {}", slug]
}

#[post("/articles/<slug>/favorite")]
pub fn favorite(_conn: DbConnection, slug: String) -> String {
    format!["Hello, {}", slug]
}

#[delete("/articles/<slug>/favorite")]
pub fn unfavorite(_conn: DbConnection, slug: String) -> String {
    format!["Hello, {}", slug]
}

#[get("/tags")]
pub fn tags(_conn: DbConnection) -> &'static str {
    "Hello, tags!"
}
