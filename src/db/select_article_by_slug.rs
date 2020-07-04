use super::article_query::ArticleQuerySql;
use diesel::pg::*;
use diesel::query_builder::*;
use diesel::sql_types::*;
use diesel::{QueryResult, RunQueryDsl};

#[derive(QueryId)]
pub struct SelectArticleBySlug {
    slug: String,
    current_user: Option<i32>,
}

pub fn select_article_by_slug(current_user: Option<i32>, slug: String) -> SelectArticleBySlug {
    SelectArticleBySlug { current_user, slug }
}

impl<'a> QueryFragment<diesel::pg::Pg> for SelectArticleBySlug {
    fn walk_ast(&self, mut out: AstPass<diesel::pg::Pg>) -> QueryResult<()> {
        out.push_sql("SELECT * FROM select_articles(");
        out.push_bind_param::<Nullable<Integer>, _>(&self.current_user)?;
        out.push_sql(", NULL, NULL) as results WHERE results.article_slug = ");
        out.push_bind_param::<Text, _>(&self.slug)?;
        out.push_sql(" LIMIT 1");
        Ok(())
    }
}

impl Query for SelectArticleBySlug {
    type SqlType = ArticleQuerySql;
}

impl RunQueryDsl<PgConnection> for SelectArticleBySlug {}
