use crate::db;
use crate::db::{DbConnection, DbResult};
use crate::errors::Error;
use crate::models::comment::*;
use crate::models::user::{Profile, User};
use crate::schema;
use ammonia;
use diesel::prelude::*;

pub fn for_article(conn: &DbConnection, user: Option<i32>, slug: &String) -> DbResult<CommentList> {
    use schema::articles;
    use schema::comments::dsl::*;
    use schema::followings;
    use schema::users;
    comments
        .inner_join(users::table)
        .inner_join(articles::table)
        .select((comments::all_columns(), users::all_columns))
        .get_results::<(CommentQuery, User)>(conn)
        .map_err(Into::<Error>::into)
        .map(|v: Vec<(CommentQuery, User)>| {
            CommentList(
                v.into_iter()
                    .map(|(comment, user): (CommentQuery, User)| {
                        comment.to_comment(user.to_profile(false))
                    })
                    .collect::<Vec<_>>(),
            )
        })
    /*
    .inner_join(articles::table)
    .inner_join(users::table)
    .left_join(
        followings::table.on(users::id
            .eq(followings::followed_id)
            .and(followings::follower_id.eq(user))),
    )
    .select((all_columns(), users::all_columns()))
    .order_by(created_at.desc())
    */
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
