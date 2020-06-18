use crate::db::DbConnection;

#[get("/articles/<slug>/comments")]
pub fn comments(_conn: DbConnection, slug: String) -> String {
    format!["Hello, {}", slug]
}

#[post("/articles/<slug>/comments")]
pub fn new_comment(_conn: DbConnection, slug: String) -> String {
    format!["Hello, {}", slug]
}

#[delete("/articles/<slug>/comments/<comment_id>")]
pub fn delete_comment(_conn: DbConnection, slug: String, comment_id: u64) -> String {
    format!["Hello, {}/{}", slug, comment_id.to_string()]
}