mod article_query;
pub mod articles;
pub mod comments;
mod get_articles;
mod get_comments;
mod limits;
mod select_article_by_slug;
mod user_feed;
pub mod users;

use crate::errors;
use diesel::connection::{Connection, SimpleConnection};
use diesel::deserialize::{Queryable, QueryableByName};
use diesel::query_builder::*;
use diesel::result::{ConnectionResult, QueryResult};
use diesel::sql_types::HasSqlType;
use rocket_contrib;
use rocket_contrib::databases::diesel;

#[database("postgres")]
pub struct DbConnection(diesel::PgConnection);

impl SimpleConnection for DbConnection {
    fn batch_execute(&self, query: &str) -> QueryResult<()> {
        self.0.batch_execute(query)
    }
}
impl Connection for DbConnection {
    fn establish(database_url: &str) -> ConnectionResult<Self> {
        diesel::r2d2::PooledConnection::establish(database_url).map(|conn| DbConnection(conn))
    }

    fn transaction_manager(&self) -> &Self::TransactionManager {
        self.0.transaction_manager()
    }

    fn execute(&self, query: &str) -> QueryResult<usize> {
        self.0.execute(query)
    }

    fn query_by_index<T, U>(&self, source: T) -> QueryResult<Vec<U>>
    where
        T: AsQuery,
        T::Query: QueryFragment<Self::Backend> + QueryId,
        Self::Backend: HasSqlType<T::SqlType>,
        U: Queryable<T::SqlType, Self::Backend>,
    {
        self.0.query_by_index(source)
    }

    fn query_by_name<T, U>(&self, source: &T) -> QueryResult<Vec<U>>
    where
        T: QueryFragment<Self::Backend> + QueryId,
        U: QueryableByName<Self::Backend>,
    {
        self.0.query_by_name(source)
    }

    fn execute_returning_count<T>(&self, source: &T) -> QueryResult<usize>
    where
        T: QueryFragment<Self::Backend> + QueryId,
    {
        self.0.execute_returning_count(source)
    }

    type Backend = <diesel::PgConnection as Connection>::Backend;
    type TransactionManager = <diesel::PgConnection as Connection>::TransactionManager;
}

pub type DbResult<T> = Result<T, errors::Error>;
