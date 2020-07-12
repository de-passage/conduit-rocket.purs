use super::limits::*;
use diesel::pg::*;
use diesel::query_builder::*;
use diesel::sql_types::*;
use diesel::{QueryResult, RunQueryDsl};

#[derive(QueryId)]
pub struct TagRequest {
    limit: i32,
}

#[derive(Queryable)]
pub struct Tag {
    pub tag: String,
    pub art_count: i64,
}

pub fn get_tags(limit: i32) -> TagRequest {
    TagRequest {
        limit: coerce_limit(Some(limit)),
    }
}

impl Query for TagRequest {
    type SqlType = (Text, BigInt);
}

impl RunQueryDsl<PgConnection> for TagRequest {}

impl QueryFragment<Pg> for TagRequest {
    fn walk_ast(&self, mut out: AstPass<Pg>) -> QueryResult<()> {
        out.push_sql(
            "SELECT tags.tag, COUNT(article_tag_associations.article_id) 
            AS art_count FROM tags 
            INNER JOIN article_tag_associations ON tag_id = tags.id 
            GROUP BY tags.tag ORDER BY art_count DESC LIMIT ",
        );
        out.push_bind_param::<Integer, _>(&self.limit)
    }
}
