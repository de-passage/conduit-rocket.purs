use crate::authentication::AuthData;
use crate::db::{DbConnection, DbResult};
use crate::errors;
use crate::models::article;
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
) -> DbResult<article::ArticleList> {
    schema::articles::table
        .load::<article::PGArticle>(conn)
        .map_err(Into::into)
        .map(to_articles)
}

pub fn tags(conn: &DbConnection) -> DbResult<article::TagList> {
    schema::tags::table
        .select(schema::tags::tag)
        .limit(20)
        .get_results(conn)
        .map_err(Into::into)
        .map(article::TagList)
}

fn to_articles(arts: Vec<article::PGArticle>) -> article::ArticleList {
    article::ArticleList(
        arts.into_iter()
            .map(
                |article::PGArticle {
                     id,
                     body,
                     created_at,
                     updated_at,
                     slug,
                     title,
                     description,
                     author,
                     favorites_count,
                 }| article::Article {
                    body,
                    slug,
                    title,
                    description,
                    favorites_count,
                    created_at: format!["{:?}", created_at],
                    updated_at: format!["{:?}", updated_at],
                    favorited: false,
                    tag_list: vec![],
                    author: crate::models::user::Profile{
                        username: String::default(),
                        bio: None,
                        image: None,
                        following: false
                    }
                },
            )
            .collect::<Vec<_>>(),
    )
}
