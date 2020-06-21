use crate::models::user::{Profile};
use chrono::NaiveDateTime;
use rocket::response::Responder;
use rocket::Request;
use rocket::response;
use crate::schema::articles;

#[derive(Serialize)]
pub struct Article {
    pub slug: String,
    pub title: String,
    pub description: String,
    pub body: String,
    #[serde(rename="tagList")]
    pub tag_list: Vec<String>,
    #[serde(rename="createdAt")]
    pub created_at: String,
    #[serde(rename="updatedAt")]
    pub updated_at: String,
    pub favorited: bool,
    #[serde(rename="favoritesCount")]
    pub favorites_count: i32,
    pub author: Profile,
}

#[derive(Queryable, Identifiable, Associations)]
#[table_name="articles"]
#[belongs_to(parent=User, foreign_key="author")]
pub struct PGArticle {
    pub id: i32,
    pub slug: String,
    pub title: String,
    pub description: String,
    pub body: String,
    pub author: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub favorites_count: i32
}

// impl diesel::associations::BelongsTo<User> for PGArticle {
//     type ForeignKeyColumn = schema::articles::author;
//     type ForeignKey = <User as Identifiable>::Id; 
//     fn foreign_key(&self) -> Option<&Self::ForeignKey> {
//         Some(&self.author)
//     }
//     fn foreign_key_column() -> Self::ForeignKeyColumn {
//         schema::articles::author
//     }
// }

#[derive(Deserialize)]
pub struct NewArticleData {
    pub title: String,
    pub description: String,
    pub body: String,
    pub tagList: Option<Vec<String>>,
}

#[derive(Deserialize)]
pub struct UpdateArticleData {
    pub title: Option<String>,
    pub description: Option<String>,
    pub body: Option<String>,
    pub tagList: Option<Vec<String>>,
}

pub struct TagList(pub Vec<String>);

impl<'r> Responder<'r> for TagList {
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        json![{ "tags": self.0 }].respond_to(req)
    }
}

pub struct ArticleList(pub Vec<Article>);

impl<'r> Responder<'r> for ArticleList {
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        json![{ "articles": self.0 }].respond_to(req)
    }
}