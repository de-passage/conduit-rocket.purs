use crate::models::user::Profile;
use chrono::NaiveDateTime;

#[derive(Serialize)]
pub struct Article {
    slug: String,
    title: String,
    description: String,
    body: String,
    tagList: Vec<String>,
    createdAt: String,
    updatedAt: String,
    favorited: bool,
    favoritesCount: i32,
    author: Profile,
}

#[derive(Queryable)]
pub struct PGArticle {
    id: i64,
    slug: String,
    title: String,
    description: String,
    body: String,
    author: i32,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
    favorites_count: i32
}

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