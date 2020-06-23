use crate::authentication::AuthData;
use crate::db;
use crate::db::DbConnection;
use crate::errors::Error;
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

#[post("/articles", data = "<data>", format = "json")]
pub fn new_article(
    conn: DbConnection,
    auth: AuthData,
    data: Json<ArticleWrapper<NewArticleData>>,
) -> DbResult<Article> {
    let article = &data.article;
    let mut errors = json![{}];
    let mut error = false;
    if article.body.is_empty() {
        errors["body"] = json!["is empty"].0;
        error = true;
    }
    if article.description.is_empty() {
        errors["description"] = json!["is empty"].0;
        error = true;
    }
    if article.title.is_empty() {
        errors["title"] = json!["is empty"].0;
        error = true;
    }

    if error {
        Err(Error::ValidationFailed(errors))
    } else {
        db::articles::create(&conn, &article, auth.id)
    }
}

#[get("/articles/feed?<limit>&<offset>")]
pub fn feed(
    conn: DbConnection,
    auth: AuthData,
    limit: Option<u32>,
    offset: Option<u32>,
) -> DbResult<ArticleList> {
    db::articles::user_feed(&conn, auth.id, limit, offset)
}

#[get("/articles/<slug>")]
pub fn article(conn: DbConnection, auth: Option<AuthData>, slug: String) -> DbResult<Article> {
    db::articles::article(&conn, auth.map(|a| a.id), &slug)
}

#[put("/articles/<slug>", data = "<data>", format = "json")]
pub fn update_article(
    conn: DbConnection,
    auth: AuthData,
    slug: String,
    data: Json<ArticleWrapper<UpdateArticleData>>,
) -> DbResult<Article> {
    let article = &data.article;
    let mut errors = json![{}];
    let mut error = false;
    match &article.body {
        Some(body) => {
            if body.is_empty() {
                errors["body"] = json!["is empty"].0;
                error = true;
            }
        }
        None => (),
    };
    match &article.description {
        Some(description) => {
            if description.is_empty() {
                errors["description"] = json!["is empty"].0;
                error = true;
            }
        }
        None => (),
    };
    match &article.title {
        Some(title) => {
            if title.is_empty() {
                errors["title"] = json!["is empty"].0;
                error = true;
            }
        }
        None => (),
    };

    if error {
        Err(Error::ValidationFailed(errors))
    } else {
        db::articles::update(&conn, auth.id, slug, &article)
    }
}

#[delete("/articles/<slug>")]
pub fn delete_article(conn: DbConnection, auth: AuthData, slug: String) -> DbResult<Article> {
    db::articles::delete(&conn, auth.id, &slug)
}

#[post("/articles/<slug>/favorite")]
pub fn favorite(conn: DbConnection, auth: AuthData, slug: String) -> DbResult<Article> {
    db::articles::favorite(&conn, auth.id, &slug)
}

#[delete("/articles/<slug>/favorite")]
pub fn unfavorite(conn: DbConnection, auth: AuthData, slug: String) -> DbResult<Article> {
    db::articles::unfavorite(&conn, auth.id, &slug)
}

#[get("/tags")]
pub fn tags(conn: DbConnection) -> DbResult<TagList> {
    db::articles::tags(&conn)
}
