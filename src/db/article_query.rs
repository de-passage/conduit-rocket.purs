use diesel::sql_types::*;

pub type ArticleQuerySql = (
    Text,
    Text,
    Text,
    Text,
    Timestamptz,
    Timestamptz,
    Text,
    Nullable<Text>,
    Nullable<Text>,
    Nullable<Array<Text>>,
    Bool,
    Bool,
    Integer,
    BigInt,
);
