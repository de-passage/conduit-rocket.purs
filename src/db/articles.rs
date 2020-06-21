use crate::authentication::AuthData;
use crate::db::{DbConnection, DbResult};
use crate::errors;
use crate::models::article::{Article, ArticleList, PGArticle, TagList};
use crate::models::user::User;
use crate::schema;
use crate::schema::articles;
use crate::schema::favorites;
use diesel::prelude::*;
use errors::Error;

pub fn articles(
    conn: &DbConnection,
    tag: Option<String>,
    author: Option<String>,
    offset: Option<u32>,
    limit: Option<u32>,
    favorited: Option<String>,
    current_user: Option<i32>,
) -> DbResult<ArticleList> {
    use schema::articles::dsl::*;
    use schema::users;
    articles
        .left_join(users::table.on(users::id.eq(id)))
        .load::<(PGArticle, Option<User>)>(conn)
        .map_err(Into::into)
        .map(|v| v.into_iter().flat_map(|p| {
            let r = to_article(p);
            r.ok().into_iter()
        }).collect::<Vec<_>>())
        .map(ArticleList)
}

pub fn tags(conn: &DbConnection) -> DbResult<TagList> {
    schema::tags::table
        .select(schema::tags::tag)
        .limit(20)
        .get_results(conn)
        .map_err(Into::into)
        .map(TagList)
}

pub fn get_by_slug(
    conn: &DbConnection,
    current_user: Option<i32>,
    search: &String,
) -> DbResult<Article> {
    use schema::articles::dsl::*;
    use schema::users;
    articles
        .filter(slug.eq(search))
        .left_join(users::table.on(users::id.eq(id)))
        .get_result::<(PGArticle, Option<User>)>(conn)
        .map_err(Into::into)
        .and_then(to_article)
}

fn to_article(tuple: (PGArticle, Option<User>)) -> DbResult<Article> {
    match tuple {
        (pg, Some(user)) => Ok(pg.to_article(user.to_profile(false))),
        (_, None) => Err(Error::DatabaseError(
            "user".to_owned(),
            "foreign key doesn't exist".to_owned(),
        )),
    }
}
