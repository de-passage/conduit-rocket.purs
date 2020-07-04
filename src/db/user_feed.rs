use super::article_query::ArticleQuerySql;
use super::limits::*;
use diesel::pg::*;
use diesel::query_builder::*;
use diesel::sql_types::*;
use diesel::{QueryResult, RunQueryDsl};

#[derive(QueryId)]
pub struct UserFeed {
    limit: i32,
    offset: i32,
    user_id: i32,
}

pub fn user_feed_query(limit: Option<i32>, offset: Option<i32>, user_id: i32) -> UserFeed {
    UserFeed {
        limit: coerce_limit(limit),
        offset: coerce_offset(offset),
        user_id,
    }
}

impl Query for UserFeed {
    type SqlType = ArticleQuerySql;
}

impl RunQueryDsl<PgConnection> for UserFeed {}

impl QueryFragment<Pg> for UserFeed {
    fn walk_ast(&self, mut out: AstPass<Pg>) -> QueryResult<()> {
        out.push_sql("SELECT * FROM user_feed(");
        out.push_bind_param::<Integer, _>(&self.user_id)?;
        out.push_sql(") LIMIT ");
        out.push_bind_param::<Integer, _>(&self.limit)?;
        out.push_sql(" OFFSET ");
        out.push_bind_param::<Integer, _>(&self.offset)?;
        Ok(())
    }
}
