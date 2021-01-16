use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Tag {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub permanent_id: u64,
    pub id: Option<String>,
    pub description: Option<String>,
    pub items_count: u64,
    pub followees_count: u64,
    pub followers_count: u64,
    pub profile_image_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QiitaResponse {
    pub body: Option<String>,
    pub created_at: Option<String>,
    pub id: String,
    pub likes_count: u64,
    pub reactions_count: u64,
    pub tags: Vec<Tag>,
    pub title: Option<String>,
    pub updated_at: Option<String>,
    pub url: Option<String>,
    pub user: User,
}
