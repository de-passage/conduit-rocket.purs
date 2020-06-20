use crate::db::DbConnection;
use crate::models;
use crate::schema;
use crate::schema::users;
use crate::authentication;
use crate::errors;
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
    username: &String,
    email: &String,
    password: &String,
) -> Result<models::user::AuthenticatedUser, errors::Error> {
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
        .map_err(Into::into)
        .and_then(|user : models::user::User| {
            match authentication::encode_token(user.id, &user.username, "secret".to_owned()) {
                Some(token) => Ok(models::user::AuthenticatedUser{
                    username: user.username,
                    bio: user.bio,
                    email: user.email,
                    image: user.image,
                    token: token
                }),
                None => Err(errors::Error::TokenError())
            }
        })
}