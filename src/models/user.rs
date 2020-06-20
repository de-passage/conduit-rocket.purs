#[derive(Serialize)]
pub struct Username(pub String);

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub username : String,
    pub email: String,
    pub bio: Option<String>,
    pub image: Option<String>,
    //#[serde(skip_serializing)]
    pub hash: String
}

#[derive(Serialize)]
pub struct AuthenticatedUser {
    pub username: String,
    pub email: String,
    pub bio: Option<String>,
    pub image: Option<String>,
    pub token: String,
    #[serde(skip_serializing)]
    pub id: i32
}

#[derive(Serialize)]
pub struct Profile {
    pub username: String,
    pub bio: Option<String>, 
    pub image: Option<String>,
    pub following: bool
}