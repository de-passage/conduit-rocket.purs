use crate::db::{DbConnection, DbResult};
use crate::errors;
use crate::models::article::{
    Article, ArticleList, NewArticleData, PGArticle, TagList, UpdateArticleData,
};
use crate::models::user::{Profile, User};
use crate::schema;
use diesel::pg::types::sql_types;
use diesel::prelude::*;
use errors::Error;
use percent_encoding::{percent_encode, NON_ALPHANUMERIC};

sql_function! {
    array_agg, ArrayAgg, (x: diesel::sql_types::Text) -> sql_types::Array<diesel::sql_types::Text>
}

pub fn articles(
    conn: &DbConnection,
    tag: Option<String>,
    author: Option<String>,
    offset: Option<u32>,
    limit: Option<u32>,
    favorited: Option<String>,
    current_user: Option<i32>,
) -> DbResult<ArticleList> {
    use schema::article_tag_associations as atas;
    use schema::articles::dsl::*;
    use schema::tags;
    use schema::users;
    articles
        .inner_join(users::table)
        .left_join(atas::table)
        .left_join(tags::table.on(tags::id.eq(atas::tag_id)))
        .select((
            articles::all_columns(),
            users::table::all_columns(),
            diesel::dsl::sql("array_agg(tags.tag)"),
        ))
        .group_by((id, users::id))
        .load(conn)
        .map_err(Into::into)
        .map(|v| {
            ArticleList(
                v.into_iter()
                    .map(|(pg, user, tags): (PGArticle, User, Option<Vec<String>>)| {
                        pg.to_article(user.to_profile(false), tags.unwrap_or(vec![]))
                    })
                    .collect::<Vec<_>>(),
            )
        })
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
        .left_join(users::table)
        .get_result::<(PGArticle, Option<User>)>(conn)
        .map_err(Into::into)
        .and_then(to_article)
}

pub fn create(conn: &DbConnection, article: &NewArticleData, user_id: i32) -> DbResult<Article> {
    use schema::articles::dsl::*;
    use schema::users;
    let profile: Profile = users::table
        .filter(users::id.eq(user_id))
        .get_result(conn)
        .map_err(Into::<Error>::into)
        .map(|u: User| u.to_profile(false))?;

    let pg_article: PGArticle = diesel::insert_into(articles)
        .values((
            slug.eq(percent_encode(article.title.as_ref(), NON_ALPHANUMERIC).collect::<String>()),
            title.eq(&article.title),
            description.eq(&article.description),
            body.eq(&article.body),
            created_at.eq(diesel::dsl::now),
            updated_at.eq(diesel::dsl::now),
            author.eq(user_id),
        ))
        .get_result(conn)
        .map_err(Into::<Error>::into)?;

    let tag_list: Vec<String> = article
        .tag_list
        .clone()
        .map(|tag_list| -> DbResult<Vec<String>> {
            use schema::tags;
            let tags = tag_list.iter().map(|t| tags::tag.eq(t)).collect::<Vec<_>>();
            diesel::insert_into(tags::table)
                .values(tags)
                .on_conflict_do_nothing()
                .execute(conn)
                .map_err(Into::<Error>::into)?;

            let ids: Vec<i32> = tags::table
                .filter(tags::tag.eq_any(&tag_list))
                .select(tags::id)
                .get_results(conn)
                .map_err(Into::<Error>::into)?;

            use schema::article_tag_associations as atas;
            diesel::insert_into(atas::table)
                .values(
                    ids.into_iter()
                        .map(|tag_id| (atas::article_id.eq(pg_article.id), atas::tag_id.eq(tag_id)))
                        .collect::<Vec<_>>(),
                )
                .execute(conn)
                .map_err(Into::<Error>::into)?;

            Ok(tag_list)
        })
        .unwrap_or(Ok(vec![]))?;

    Ok(pg_article.to_article(profile, tag_list))
}

fn to_article(tuple: (PGArticle, Option<User>)) -> DbResult<Article> {
    match tuple {
        (pg, Some(user)) => Ok(pg.to_article(user.to_profile(false), vec![])),
        (_, None) => Err(Error::DatabaseError(
            "user".to_owned(),
            "foreign key doesn't exist".to_owned(),
        )),
    }
}
