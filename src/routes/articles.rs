use crate::db::DbConnection;

#[get("/articles?<tag>&<author>&<offset>&<limit>&<favorited>")]
pub fn articles(
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
