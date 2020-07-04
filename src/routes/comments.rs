use crate::authentication::AuthData;
use crate::db;
use crate::db::{DbConnection, DbResult};
use crate::errors::Error;
use crate::models::comment::{Comment, CommentList, NewCommentData};
use rocket_contrib::json::Json;

#[derive(Deserialize)]
pub struct CommentWrapper {
    comment: NewCommentData,
}

#[get("/articles/<slug>/comments")]
pub fn comments(conn: DbConnection, auth: Option<AuthData>, slug: String) -> DbResult<CommentList> {
    db::comments::for_article(&conn, auth.map(|a| a.id), slug)
}

#[post("/articles/<slug>/comments", data = "<comment>", format = "json")]
pub fn new_comment(
    conn: DbConnection,
    auth: AuthData,
    slug: String,
    comment: Json<CommentWrapper>,
) -> DbResult<Comment> {
    if comment.comment.body.is_empty() {
        Err(Error::ValidationFailed(json![{"body": "is empty"}]))
    } else {
        db::comments::create(&conn, auth.id, &slug, &comment.comment)
    }
}

#[delete("/articles/<slug>/comments/<comment_id>")]
pub fn delete_comment(
    conn: DbConnection,
    auth: AuthData,
    slug: String,
    comment_id: i32,
) -> DbResult<Comment> {
    db::comments::delete(&conn, auth.id, &slug, comment_id)
}
