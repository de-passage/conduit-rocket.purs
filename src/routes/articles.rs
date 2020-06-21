use crate::db::DbConnection;
use crate::authentication::AuthData;
use crate::models;
use crate::models::article::{TagList, Article, ArticleList};
use crate::db;
use db::DbResult;

#[get("/articles?<tag>&<author>&<offset>&<limit>&<favorited>")]
pub fn articles(
    conn: DbConnection,
    auth: Option<AuthData>,
    tag: Option<String>,
    author: Option<String>,
    offset: Option<u32>,
    limit: Option<u32>,
    favorited: Option<String>,
) -> DbResult<ArticleList> {
    db::articles::articles(&conn, tag, author, offset, limit, favorited, auth.map(|a| a.id))
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
pub fn article(conn: DbConnection, auth: Option<AuthData>, slug: String) -> DbResult<Article> {
    db::articles::get_by_slug(&conn, auth.map(|a| a.id), &slug)
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
pub fn tags(conn: DbConnection) -> DbResult<TagList> {
    db::articles::tags(&conn)
}
