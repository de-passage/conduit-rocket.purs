use crate::db::{DbConnection, DbResult};
use crate::errors;
use crate::models::article::{
    Article, ArticleList, NewArticleData, PGArticle, TagList, UpdateArticleData,
};
use crate::models::user::{Profile, User};
use crate::schema;
use ammonia;
use diesel::prelude::*;
use errors::Error;
use rand::distributions::Alphanumeric;
use rand::*;
use slug;
use std::cmp::max;

const LIMIT: u32 = 20;
const MAX_LIMIT: i64 = 500;
const SUFFIX_LEN: usize = 8;

pub fn articles(
    conn: &DbConnection,
    m_tag: Option<String>,
    m_author: Option<String>,
    m_offset: Option<u32>,
    m_limit: Option<u32>,
    m_favorited: Option<String>,
    current_user: Option<i32>,
) -> DbResult<ArticleList> {
    use schema::article_tag_associations as atas;
    use schema::articles::dsl::*;
    use schema::tags;
    use schema::users;

    let mut query = articles
        .inner_join(users::table)
        .left_join(atas::table)
        .left_join(tags::table.on(tags::id.eq(atas::tag_id)))
        .left_join(schema::favorites::table)
        .select((
            articles::all_columns(),
            users::table::all_columns(),
            diesel::dsl::sql("array_agg(tags.tag) as tag_list"),
        ))
        .group_by((id, users::id))
        .into_boxed();
    if let Some(auth) = m_author {
        let author_id = users::table
            .filter(users::username.eq(auth))
            .select(users::id)
            .get_result::<i32>(conn)
            .map_err(Into::<Error>::into)?;
        query = query.filter(author.eq(author_id));
    }

    query
        .offset(m_offset.unwrap_or(0).into())
        .limit(max(m_limit.unwrap_or(LIMIT).into(), MAX_LIMIT))
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
        .inner_join(users::table)
        .get_result::<(PGArticle, User)>(conn)
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
            slug.eq(slugify(&article.title)),
            title.eq(&ammonia::clean(&article.title)),
            description.eq(&ammonia::clean(&article.description)),
            body.eq(&ammonia::clean(&article.body)),
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

fn to_article((pg, user): (PGArticle, User)) -> DbResult<Article> {
    Ok(pg.to_article(user.to_profile(false), vec![]))
}

fn slugify(title: &str) -> String {
    format!("{}-{}", slug::slugify(title), generate_suffix(SUFFIX_LEN))
}

fn generate_suffix(len: usize) -> String {
    let mut rng = thread_rng();
    (0..len).map(|_| rng.sample(Alphanumeric)).collect()
}
