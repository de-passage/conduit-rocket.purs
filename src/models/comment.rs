use crate::models::user;
use chrono::NaiveDateTime;

#[derive(Serialize)]
pub struct Comment {
    pub id: i32,
    pub author: user::Profile,
    pub createdAt: String,
    pub updatedAt: String,
    pub body: String,
}

#[derive(Queryable)]
pub struct CommentQuery { 
    pub id: i32,
    pub author: i32,
    pub body: String,
    pub createdAt: NaiveDateTime,
    pub updatedAt: NaiveDateTime,
}

#[derive(Deserialize)]
pub struct NewCommentData {
    pub body: String
}