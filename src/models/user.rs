#[derive(Serialize)]
pub struct Username(pub String);

#[derive(Serialize)]
pub struct User {
    pub username : Username,
    pub bio: Option<String>,
    pub image: Option<String>,
    pub id: u32,
    pub email: String,
    pub token: String
}

#[derive(Serialize)]
pub struct Profile {
    pub username: Username,
    pub bio: Option<String>, 
    pub image: Option<String>,
    pub following: bool
}