use crate::authentication;
use crate::db::DbResult;

#[derive(Serialize)]
pub struct Username(pub String);

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub bio: Option<String>,
    pub image: Option<String>,
    //#[serde(skip_serializing)]
    pub hash: String,
}

#[derive(Serialize)]
pub struct AuthenticatedUser {
    pub username: String,
    pub email: String,
    pub bio: Option<String>,
    pub image: Option<String>,
    pub token: String,
    #[serde(skip_serializing)]
    pub id: i32,
}

#[derive(Serialize)]
pub struct Profile {
    pub username: String,
    pub bio: Option<String>,
    pub image: Option<String>,
    pub following: bool,
}

impl User {
    pub fn to_profile(self, followed: bool) -> Profile {
        Profile {
            username: self.username,
            bio: self.bio,
            image: self.image,
            following: followed,
        }
    }

    pub fn to_authenticated(self, secret: &String) -> DbResult<AuthenticatedUser> {
        authentication::encode_token(self.id, &self.username, secret).map(|token| {
            AuthenticatedUser {
                username: self.username,
                bio: self.bio,
                email: self.email,
                image: self.image,
                token: token,
                id: self.id,
            }
        })
    }
}

#[derive(Deserialize)]
pub struct NewUserData {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct UserUpdateData {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub bio: Option<String>,
    pub image: Option<String>,
}

#[derive(Deserialize)]
pub struct LoginData {
    pub email: String,
    pub password: String,
}
