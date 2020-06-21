use crate::authentication::AuthData;
use crate::db;
use crate::db::DbConnection;
use crate::models::article::{Article, ArticleList, NewArticleData, TagList, UpdateArticleData};
use db::DbResult;
use rocket_contrib::json::Json;

#[derive(Deserialize)]
pub struct ArticleWrapper<T> {
    article: T,
}

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
    db::articles::articles(
        &conn,
        tag,
        author,
        offset,
        limit,
        favorited,
        auth.map(|a| a.id),
    )
}

#[post("/articles", data = "<article>", format = "json")]
pub fn new_article(
    conn: DbConnection,
    auth: AuthData,
    article: Json<ArticleWrapper<NewArticleData>>,
) -> DbResult<Article> {
    db::articles::create(&conn, &article.article, auth.id)
}

#[get("/articles/feed?<limit>&<offset>")]
pub fn feed(_conn: DbConnection, limit: Option<u32>, offset: Option<u32>) {}

#[get("/articles/<slug>")]
pub fn article(conn: DbConnection, auth: Option<AuthData>, slug: String) -> DbResult<Article> {
    db::articles::get_by_slug(&conn, auth.map(|a| a.id), &slug)
}

#[put("/articles/<slug>", data = "<article>", format = "json")]
pub fn update_article(
    _conn: DbConnection,
    auth: AuthData,
    slug: String,
    article: Json<ArticleWrapper<UpdateArticleData>>,
) -> String {
    format!["Hello, {}", slug]
}

#[delete("/articles/<slug>")]
pub fn delete_article(_conn: DbConnection, auth: AuthData, slug: String) -> String {
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
