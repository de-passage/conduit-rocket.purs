use crate::authentication;
use crate::db::{DbConnection, DbResult};
use crate::errors;
use crate::models::user::{AuthenticatedUser, Profile, User};
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
    conn: &DbConnection,
    username: &String,
    email: &String,
    password: &String,
) -> DbResult<AuthenticatedUser> {
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
        .get_result(conn)
        .map_err(Into::into)
        .and_then(to_authenticated)
}

pub fn authenticate(
    conn: &DbConnection,
    email: &String,
    password: &String,
) -> DbResult<AuthenticatedUser> {
    schema::users::table
        .filter(users::email.eq(email))
        .get_result(conn)
        .map_err(Into::into)
        .and_then(|user: User| {
            scrypt::scrypt_check(password, &user.hash)
                .map_err(|_| errors::Error::AuthError())
                .and_then(|_| to_authenticated(user))
        })
}

pub fn profile(
    conn: &DbConnection,
    username: &String,
    current_user: &Option<AuthenticatedUser>,
) -> DbResult<Profile> {
    schema::users::table
        .filter(users::username.eq(username))
        .get_result(conn)
        .map_err(Into::into)
        .and_then(|user: User| match current_user {
            None => Ok(to_profile(user, false)),
            Some(current) => {
                if current.id == user.id {
                    Ok(to_profile(user, false))
                } else {
                    let followed: bool = schema::followings::table
                        .filter(schema::followings::follower_id
                            .eq(current.id)
                            .and(schema::followings::followed_id.eq(user.id)))
                        .count()
                        .first(conn)
                        .map_or(false, |x : i64| x > 0);
                    Ok(to_profile(user, followed))
                }
            }
        })
}

fn to_profile(user: User, followed: bool) -> Profile {
    Profile {
        username: user.username,
        bio: user.bio,
        image: user.image,
        following: followed,
    }
}

fn to_authenticated(user: User) -> DbResult<AuthenticatedUser> {
    match authentication::encode_token(user.id, &user.username, "secret".to_owned()) {
        Some(token) => Ok(AuthenticatedUser {
            username: user.username,
            bio: user.bio,
            email: user.email,
            image: user.image,
            token: token,
            id: user.id,
        }),
        None => Err(errors::Error::TokenError()),
    }
}
