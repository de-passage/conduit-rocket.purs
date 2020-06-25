use crate::authentication::AuthData;
use crate::db::{DbConnection, DbResult};
use crate::errors;
use crate::models::user::{AuthenticatedUser, Profile, User, UserUpdateData};
use crate::schema;
use crate::schema::followings;
use crate::schema::users;
use ammonia;
use errors::Error;
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
    let hash = make_hash(password)?;

    diesel::insert_into(users::table)
        .values(NewUserData {
            username: &ammonia::clean(&username),
            email: &ammonia::clean(&email),
            hash: &ammonia::clean(&hash),
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
        .optional()
        .map_err(Into::into)
        .and_then(|maybe_user: Option<User>| match maybe_user {
            Some(user) => scrypt::scrypt_check(password, &user.hash)
                .map_err(|_| Error::AuthError)
                .and_then(|_| user.to_authenticated(secret)),
            None => Err(Error::ValidationFailed(json![{"email": "doesn't exist"}])),
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
                let followed: bool = followings::table
                    .filter(
                        followings::follower_id
                            .eq(current.id)
                            .and(followings::followed_id.eq(user.id)),
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

pub fn update(
    conn: &DbConnection,
    id: i32,
    upd: &UserUpdateData,
    secret: &String,
) -> DbResult<AuthenticatedUser> {
    let data = UpdateUserData {
        username: upd.username.clone().map(|a| ammonia::clean(&a)),
        email: upd.email.clone().map(|a| ammonia::clean(&a)),
        hash: upd.password.clone().clone().and_then(|v| make_hash(v).ok()),
        image: upd.image.clone().map(|a| ammonia::clean(&a)),
        bio: upd.bio.clone().map(|a| ammonia::clean(&a)),
    };

    diesel::update(users::table.filter(users::id.eq(id)))
        .set(data)
        .get_result(conn)
        .map_err(Into::into)
        .and_then(|u: User| u.to_authenticated(secret))
}

pub fn follow(conn: &DbConnection, username: &String, id: i32) -> DbResult<Profile> {
    use followings::{followed_id, follower_id};
    let user = find_by_username(conn, username)?;
    diesel::insert_into(followings::table)
        .values((follower_id.eq(id), followed_id.eq(user.id)))
        .execute(conn)
        .map_err(Into::into)
        .and_then(|inserted| {
            if inserted > 0 {
                Ok(user.to_profile(true))
            } else {
                Err(Error::InternalServerError(
                    "followings".to_owned(),
                    "couldn't follow user".to_owned(),
                ))
            }
        })
}

pub fn unfollow(conn: &DbConnection, username: &String, id: i32) -> DbResult<Profile> {
    use followings::{followed_id, follower_id};
    let user = find_by_username(conn, username)?;
    diesel::delete(followings::table.filter(follower_id.eq(id).and(followed_id.eq(user.id))))
        .execute(conn)
        .map_err(Into::into)
        .and_then(|deleted| {
            if deleted != 1 {
                Err(Error::InternalServerError(
                    "followings".to_owned(),
                    "couldn't unfollow user".to_owned(),
                ))
            } else {
                Ok(user.to_profile(false))
            }
        })
}

impl From<scrypt::errors::InvalidParams> for Error {
    fn from(err: scrypt::errors::InvalidParams) -> Error {
        Error::InternalServerError("password".to_owned(), err.to_string())
    }
}

fn make_hash(password: String) -> Result<String, Error> {
    scrypt::ScryptParams::new(10, 8, 1)
        .map_err(Into::into)
        .and_then(|value| {
            scrypt::scrypt_simple(&password, &value)
                .map_err(|err| Error::InternalServerError("password".to_owned(), err.to_string()))
        })
}
