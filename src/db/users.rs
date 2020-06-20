use crate::authentication::AuthData;
use crate::db::{DbConnection, DbResult};
use crate::errors;
use crate::models::user::{AuthenticatedUser, Profile, User, UserUpdateData};
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

#[derive(AsChangeset)]
#[table_name = "users"]
struct UpdateUserData {
    username: Option<String>,
    email: Option<String>, 
    bio: Option<String>,
    image: Option<String>,
    hash: Option<String>,
}

pub fn create(
    conn: &DbConnection,
    username: &String,
    email: &String,
    password: String,
    secret: &String,
) -> DbResult<AuthenticatedUser> {
    let hash = make_hash(password);

    diesel::insert_into(schema::users::table)
        .values(NewUserData {
            username: &username,
            email: &email,
            hash: &hash,
        })
        .get_result(conn)
        .map_err(Into::into)
        .and_then(|u: User| u.to_authenticated(secret))
}

pub fn authenticate(
    conn: &DbConnection,
    email: &String,
    password: &String,
    secret: &String,
) -> DbResult<AuthenticatedUser> {
    schema::users::table
        .filter(users::email.eq(email))
        .get_result(conn)
        .map_err(Into::into)
        .and_then(|user: User| {
            scrypt::scrypt_check(password, &user.hash)
                .map_err(|_| errors::Error::AuthError())
                .and_then(|_| user.to_authenticated(secret))
        })
}

pub fn profile(
    conn: &DbConnection,
    username: &String,
    current_user: &Option<AuthData>,
) -> DbResult<Profile> {
    find_by_username(conn, username).and_then(|user: User| match current_user {
        None => Ok(user.to_profile(false)),
        Some(current) => {
            if current.id == user.id {
                Ok(user.to_profile(false))
            } else {
                let followed: bool = schema::followings::table
                    .filter(
                        schema::followings::follower_id
                            .eq(current.id)
                            .and(schema::followings::followed_id.eq(user.id)),
                    )
                    .count()
                    .first(conn)
                    .map_or(false, |x: i64| x > 0);
                Ok(user.to_profile(followed))
            }
        }
    })
}

pub fn find_by_username(conn: &DbConnection, username: &String) -> DbResult<User> {
    schema::users::table
        .filter(users::username.eq(username))
        .get_result(conn)
        .map_err(Into::into)
}

pub fn find_by_id(conn: &DbConnection, id: i32) -> DbResult<User> {
    schema::users::table
        .filter(users::id.eq(id))
        .get_result(conn)
        .map_err(Into::into)
}

pub fn update(conn: &DbConnection, id: i32, upd: &UserUpdateData, secret: &String) -> DbResult<AuthenticatedUser> {
    let data = UpdateUserData {
        username : upd.username.clone(),
        email : upd.username.clone(),
        hash : upd.password.clone().map(make_hash),
        image: upd.image.clone(),
        bio: upd.bio.clone()
    };

    diesel::update(users::table.filter(users::id.eq(id)))
        .set(data)
        .get_result(conn)
        .map_err(Into::into)
        .and_then(|u : User| u.to_authenticated(secret))
}

fn make_hash(password: String) -> String {
    scrypt::scrypt_simple(
        &password,
        &scrypt::ScryptParams::new(14, 8, 1).expect("Invalid parameters"),
    )
    .expect("Error hashing password")
}