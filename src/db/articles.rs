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
use std::cmp::min;

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
        .limit(min(m_limit.unwrap_or(LIMIT).into(), MAX_LIMIT))
        .order_by(created_at.desc())
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

pub fn article(
    conn: &DbConnection,
    current_user: Option<i32>,
    search: &String,
) -> DbResult<Article> {
    get_by_slug(conn, current_user, search).map(to_article)
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

pub fn delete(conn: &DbConnection, user_id: i32, to_delete: &String) -> DbResult<Article> {
    use schema::articles::dsl::*;
    let (author_id, art_id): (i32, i32) = articles
        .select((author, id))
        .filter(slug.eq(to_delete))
        .get_result(conn)
        .map_err(Into::<Error>::into)?;
    if author_id != user_id {
        return Err(Error::Unauthorized);
    }
    let artcl = get_by_slug(conn, Some(user_id), to_delete).map(to_article)?;
    diesel::delete(articles)
        .filter(id.eq(art_id))
        .execute(conn)
        .map_err(Into::<Error>::into)?;
    Ok(artcl)
}

use schema::articles;
#[derive(AsChangeset)]
#[table_name = "articles"]
struct ChangeArticle {
    slug: Option<String>,
    title: Option<String>,
    description: Option<String>,
    body: Option<String>,
}

pub fn update(
    conn: &DbConnection,
    user_id: i32,
    to_update: String,
    data: &UpdateArticleData,
) -> DbResult<Article> {
    use schema::articles::dsl::*;
    let art_id: i32 = articles
        .filter(slug.eq(&to_update).and(author.eq(user_id)))
        .select(id)
        .get_result(conn)
        .map_err(|_| Error::Forbidden)?;
    let new_slug = data.title.clone().map(|a| ammonia::clean(&a));
    diesel::update(articles)
        .filter(id.eq(art_id))
        .set((
            ChangeArticle {
                slug: new_slug.clone(),
                title: data.title.clone().map(|a| ammonia::clean(&a)),
                description: data.description.clone().map(|a| ammonia::clean(&a)),
                body: data.body.clone().map(|a| ammonia::clean(&a)),
            },
            updated_at.eq(diesel::dsl::now),
        ))
        .execute(conn)
        .map_err(Into::<Error>::into)?;
    get_by_slug(conn, Some(user_id), &new_slug.unwrap_or(to_update)).map(to_article)
}

pub fn favorite(conn: &DbConnection, favoriter: i32, fav: &String) -> DbResult<Article> {
    let mut art = get_by_slug(conn, Some(favoriter), fav)?;
    use schema::favorites::dsl::*;
    diesel::insert_into(favorites)
        .values((user_id.eq(favoriter), article_id.eq(art.0.id)))
        .execute(conn)
        .map_err(Into::<Error>::into)?;
    art.0.favorites_count += 1;
    let updated = diesel::update(articles::table)
        .filter(articles::id.eq(art.0.id))
        .set(articles::favorites_count.eq(art.0.favorites_count))
        .execute(conn)?;
    if updated == 1 {
        Ok(to_article(art))
    } else {
        Err(Error::DatabaseError(
            "article".to_owned(),
            "favorites_count update failed".to_owned(),
        ))
    }
}

pub fn unfavorite(conn: &DbConnection, favoriter: i32, fav: &String) -> DbResult<Article> {
    let mut art = get_by_slug(conn, Some(favoriter), fav)?;
    use schema::favorites::dsl::*;
    let deleted = diesel::delete(favorites)
        .filter(user_id.eq(favoriter).and(article_id.eq(art.0.id)))
        .execute(conn)
        .map_err(Into::<Error>::into)?;
    if deleted > 0 {
        art.0.favorites_count -= 1;
        let updated = diesel::update(articles::table)
            .filter(articles::id.eq(art.0.id))
            .set(articles::favorites_count.eq(art.0.favorites_count))
            .execute(conn)?;
        if updated == 1 {
            Ok(to_article(art))
        } else {
            Err(Error::DatabaseError(
                "article".to_owned(),
                "favorites_count update failed".to_owned(),
            ))
        }
    } else {
        Err(Error::DatabaseError(
            "article".to_owned(),
            "wasn't favorited".to_owned(),
        ))
    }
}

pub fn user_feed(
    conn: &DbConnection,
    user_id: i32,
    limit: Option<u32>,
    offset: Option<u32>,
) -> DbResult<ArticleList> {
    use schema::article_tag_associations as atas;
    use schema::articles::dsl::*;
    use schema::followings;
    use schema::tags;
    use schema::users;
    articles
        .inner_join(users::table)
        .left_join(followings::table.on(users::id.eq(followings::followed_id)))
        .left_join(atas::table)
        .left_join(tags::table.on(atas::tag_id.eq(tags::id)))
        .filter(followings::follower_id.eq(user_id))
        .select((
            articles::all_columns(),
            users::table::all_columns(),
            tags_as_array(),
        ))
        .group_by((id, users::id))
        .limit(min(limit.unwrap_or(LIMIT).into(), MAX_LIMIT))
        .offset(offset.unwrap_or(0).into())
        .get_results::<(PGArticle, User, Option<Vec<String>>)>(conn)
        .map(|v| ArticleList(v.into_iter().map(to_article).collect::<Vec<Article>>()))
        .map_err(Into::<Error>::into)
}

fn slugify(title: &str) -> String {
    format!("{}-{}", slug::slugify(title), generate_suffix(SUFFIX_LEN))
}

fn generate_suffix(len: usize) -> String {
    let mut rng = thread_rng();
    (0..len).map(|_| rng.sample(Alphanumeric)).collect()
}

fn to_article((pg, user, tags): (PGArticle, User, Option<Vec<String>>)) -> Article {
    pg.to_article(user.to_profile(false), tags.unwrap_or(vec![]))
}

fn tags_as_array<ST>() -> diesel::expression::SqlLiteral<ST> {
    diesel::dsl::sql("array_agg(tags.tag) as tag_list")
}

fn get_by_slug(
    conn: &DbConnection,
    current_user: Option<i32>,
    search: &String,
) -> DbResult<(PGArticle, User, Option<Vec<String>>)> {
    use schema::article_tag_associations as atas;
    use schema::articles::dsl::*;
    use schema::tags;
    use schema::users;
    articles
        .filter(slug.eq(search))
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
        .get_result::<(PGArticle, User, Option<Vec<String>>)>(conn)
        .map_err(Into::into)
}
