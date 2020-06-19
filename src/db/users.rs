use crate::db::DbConnection;
use crate::models;
use crate::schema;
use crate::schema::users;
use scrypt;

use diesel::prelude::*;

#[derive(Insertable)]
#[table_name = "users"]
struct NewUserData<'a> {
    username: &'a str,
    email: &'a str,
    hash: &'a str,
}

pub fn create(
    conn: DbConnection,
    username: String,
    email: String,
    password: String,
) -> Result<models::user::User, diesel::result::Error> {
    let hash = scrypt::scrypt_simple(
        &password,
        &scrypt::ScryptParams::new(14, 8, 1).expect("Invalid parameters"),
    )
    .expect("Error hashing password");
    diesel::insert_into(schema::users::table)
        .values(NewUserData {
            username: &username,
            email: &email,
            hash: &hash,
        })
        .get_result(&conn.0)
}
