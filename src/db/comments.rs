use crate::db;
use crate::db::get_comments::*;
use crate::db::{DbConnection, DbResult};
use crate::errors::Error;
use crate::models::comment::*;
use crate::models::user::Profile;
use crate::schema;
use ammonia;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::sql_types::*;

#[derive(QueryableByName, Queryable)]
struct CommentQ {
    #[sql_type = "Integer"]
    comment_id: i32,
    #[sql_type = "Text"]
    comment_body: String,
    #[sql_type = "Timestamptz"]
    comment_creation: NaiveDateTime,
    #[sql_type = "Timestamptz"]
    comment_update: NaiveDateTime,
    #[sql_type = "Text"]
    author_username: String,
    #[sql_type = "Nullable<Text>"]
    author_bio: Option<String>,
    #[sql_type = "Nullable<Text>"]
    author_image: Option<String>,
    #[sql_type = "Bool"]
    is_followed: bool,
    #[sql_type = "BigInt"]
    total_comments: i64,
}

pub fn for_article(conn: &DbConnection, user: Option<i32>, slug: String) -> DbResult<CommentList> {
    get_comments(user, slug)
        .get_results::<CommentQ>(conn)
        .map_err(Into::<Error>::into)
        .map(|v: Vec<CommentQ>| CommentList {
            comments_count: (&v).first().map(|x| x.total_comments).unwrap_or(0),
            comments: v
                .into_iter()
                .map(|comment: CommentQ| Comment {
                    id: comment.comment_id,
                    body: comment.comment_body,
                    created_at: format!["{:?}", comment.comment_creation],
                    updated_at: format!["{:?}", comment.comment_update],
                    author: Profile {
                        username: comment.author_username,
                        bio: comment.author_bio,
                        image: comment.author_image,
                        following: comment.is_followed,
                    },
                })
                .collect::<Vec<_>>(),
        })
}

pub fn create(
    conn: &DbConnection,
    user: i32,
    slug: &String,
    comment: &NewCommentData,
) -> DbResult<Comment> {
    use schema::comments::dsl::*;
    let article = find_article_id(conn, slug)?;
    let profile = db::users::find_by_id(conn, user)?.to_profile(false);

    diesel::insert_into(comments)
        .values((
            user_id.eq(user),
            article_id.eq(article),
            created_at.eq(diesel::dsl::now),
            updated_at.eq(diesel::dsl::now),
            body.eq(&ammonia::clean(&comment.body)),
        ))
        .get_result::<CommentQuery>(conn)
        .map(|q| q.to_comment(profile))
        .map_err(Into::<Error>::into)
}

pub fn delete(conn: &DbConnection, user: i32, _: &String, comment_id: i32) -> DbResult<Comment> {
    use schema::comments::dsl::*;

    let qcomment: CommentQuery = diesel::delete(comments)
        .filter(id.eq(comment_id).and(user_id.eq(user)))
        .get_result(conn)
        .map_err(Into::<Error>::into)?;

    let u = db::users::find_by_id(conn, user)?;
    Ok(qcomment.to_comment(u.to_profile(false)))
}

fn find_article_id(conn: &DbConnection, article_slug: &String) -> DbResult<i32> {
    use schema::articles::dsl::*;
    articles
        .filter(slug.eq(article_slug))
        .select(id)
        .get_result::<i32>(conn)
        .map_err(Into::<Error>::into)
}
