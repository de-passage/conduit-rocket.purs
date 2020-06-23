use crate::models::user::Profile;
use crate::schema::articles;
use chrono::NaiveDateTime;
use rocket::response;
use rocket::response::Responder;
use rocket::Request;
use slug;

use rand::distributions::Alphanumeric;
use rand::*;
const SUFFIX_LEN: usize = 8;

#[derive(Serialize)]
pub struct Article {
    pub slug: String,
    pub title: String,
    pub description: String,
    pub body: String,
    #[serde(rename = "tagList")]
    pub tag_list: Vec<String>,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    pub favorited: bool,
    #[serde(rename = "favoritesCount")]
    pub favorites_count: i32,
    pub author: Profile,
}

#[derive(Queryable, Identifiable, Associations)]
#[table_name = "articles"]
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
    pub favorites_count: i32,
}

impl PGArticle {
    pub fn to_article(self, profile: Profile, tag_list: Vec<String>, favorited: bool) -> Article {
        let PGArticle {
            body,
            created_at,
            updated_at,
            slug,
            title,
            description,
            favorites_count,
            ..
        } = self;
        Article {
            body,
            slug,
            title,
            description,
            favorites_count,
            created_at: format!["{:?}", created_at],
            updated_at: format!["{:?}", updated_at],
            favorited,
            tag_list,
            author: profile,
        }
    }
}

#[derive(Deserialize)]
pub struct NewArticleData {
    pub title: String,
    pub description: String,
    pub body: String,
    #[serde(rename = "tagList")]
    pub tag_list: Option<Vec<String>>,
}

#[derive(Deserialize)]
pub struct UpdateArticleData {
    pub title: Option<String>,
    pub description: Option<String>,
    pub body: Option<String>,
    #[serde(rename = "tagList")]
    pub tag_list: Option<Vec<String>>,
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

impl<'r> Responder<'r> for Article {
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        json![{ "article": self }].respond_to(req)
    }
}

pub fn slugify(title: &str) -> String {
    format!("{}-{}", slug::slugify(title), generate_suffix(SUFFIX_LEN))
}

fn generate_suffix(len: usize) -> String {
    let mut rng = thread_rng();
    (0..len).map(|_| rng.sample(Alphanumeric)).collect()
}
