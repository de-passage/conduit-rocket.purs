use super::article_query::*;
use super::limits::*;
use diesel::pg::*;
use diesel::query_builder::*;
use diesel::sql_types::*;
use diesel::QueryResult;
use diesel::RunQueryDsl;

#[derive(QueryId)]
pub struct GetArticles {
    limit: i32,
    offset: i32,
    current_user: Option<i32>,
    tag: Option<String>,
    favorited: Option<String>,
    author: Option<String>,
}

impl QueryFragment<Pg> for GetArticles {
    fn walk_ast(&self, mut out: AstPass<Pg>) -> QueryResult<()> {
        out.push_sql("SELECT * FROM get_articles(");
        out.push_bind_param::<Integer, _>(&self.limit)?;
        out.push_sql(", ");
        out.push_bind_param::<Integer, _>(&self.offset)?;
        out.push_sql(", ");
        out.push_bind_param::<Nullable<Integer>, _>(&self.current_user)?;
        out.push_sql(", ");
        out.push_bind_param::<Nullable<Text>, _>(&self.tag)?;
        out.push_sql(", ");
        out.push_bind_param::<Nullable<Text>, _>(&self.favorited)?;
        out.push_sql(", ");
        out.push_bind_param::<Nullable<Text>, _>(&self.author)?;
        out.push_sql(")");
        Ok(())
    }
}

impl Query for GetArticles {
    type SqlType = ArticleQuerySql;
}

impl RunQueryDsl<PgConnection> for GetArticles {}

pub fn get_articles(
    limit: Option<i32>,
    offset: Option<i32>,
    current_user: Option<i32>,
    tag: Option<String>,
    favorited: Option<String>,
    author: Option<String>,
) -> GetArticles {
    GetArticles {
        limit: coerce_limit(limit),
        offset: coerce_limit(offset),
        current_user,
        tag,
        favorited,
        author,
    }
}
