use crate::format::encode_datetime;
use crate::models::user;
use chrono::NaiveDateTime;
use rocket::response;
use rocket::response::Responder;
use rocket::Request;

#[derive(Serialize)]
pub struct Comment {
    pub id: i32,
    pub author: user::Profile,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    pub body: String,
}

#[derive(Queryable)]
pub struct CommentQuery {
    pub id: i32,
    pub body: String,
    pub user_id: i32,
    pub article_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl CommentQuery {
    pub fn to_comment(self, author: user::Profile) -> Comment {
        Comment {
            id: self.id,
            author,
            body: self.body,
            updated_at: encode_datetime(self.updated_at),
            created_at: encode_datetime(self.created_at),
        }
    }
}

#[derive(Deserialize)]
pub struct NewCommentData {
    pub body: String,
}

pub struct CommentList {
    pub comments: Vec<Comment>,
    pub comments_count: i64,
}

impl<'r> Responder<'r> for CommentList {
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        json![{ "comments": self.comments, "commentsCount": self.comments_count }].respond_to(req)
    }
}

impl<'r> Responder<'r> for Comment {
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        json![{ "comment": self }].respond_to(req)
    }
}
