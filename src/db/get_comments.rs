use diesel::pg::*;
use diesel::query_builder::*;
use diesel::sql_types::*;
use diesel::QueryResult;

pub type CommentSql = (
    Integer,
    Text,
    Timestamptz,
    Timestamptz,
    Text,
    Nullable<Text>,
    Nullable<Text>,
    Bool,
    BigInt,
);

#[derive(QueryId)]
pub struct GetComments {
    slug: String,
    user: Option<i32>,
}

impl QueryFragment<Pg> for GetComments {
    fn walk_ast(&self, mut out: AstPass<Pg>) -> QueryResult<()> {
        out.push_sql("SELECT * FROM get_comments(");
        out.push_bind_param::<Text, _>(&self.slug)?;
        out.push_sql(", ");
        out.push_bind_param::<Nullable<Integer>, _>(&self.user)?;
        out.push_sql(")");
        Ok(())
    }
}

impl diesel::query_builder::Query for GetComments {
    type SqlType = CommentSql;
}

impl diesel::RunQueryDsl<diesel::pg::PgConnection> for GetComments {}

pub fn get_comments(user: Option<i32>, slug: String) -> GetComments {
    GetComments { slug, user }
}
