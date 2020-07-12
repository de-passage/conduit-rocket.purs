#[macro_use]
extern crate diesel;

#[path = "../schema.rs"]
mod schema;

use diesel::dsl::*;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

fn main() {
    if cfg!(debug_assertions) {
        dotenv().ok();
    }

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let connection = PgConnection::establish(&database_url).expect(&format!(
        "Database connection failed. Url: {}",
        database_url
    ));

    {
        use schema::users::dsl::*;
        delete(users.filter(username.ne(all(vec!["Admin", "Sylvain Leclercq"]))))
            .execute(&connection)
            .expect("Deletion failed");
    }
}
