use crate::note;
use note::NoteResponse;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum ServiceName {
    Qiita,
    Note,
    GitHub,
    Twitter,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    id: u64,
    user_name: Option<String>,
    display_name: Option<String>,
    avatar_url: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Article {
    id: u64,
    title: Option<String>,
    created_at: Option<String>,
    body: Option<String>,
    image_url: Option<String>,
    tags: Vec<String>,
    user: User,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArticleResponse {
    pub service_name: ServiceName,
    pub articles: Vec<Article>,
    pub summary: String,
}

impl ArticleResponse {
    pub fn from_note(service_name: ServiceName, note_response: NoteResponse) -> Self {
        let articles = note_response
            .data
            .contents
            .into_iter()
            .map(|content| Article {
                id: content.id,
                title: content.name,
                created_at: content.publishAt,
                body: content.body,
                image_url: content.eyecatch,
                tags: content
                    .hashtags
                    .into_iter()
                    .map(|tag| tag.hashtag.name)
                    .collect(),
                user: User {
                    id: content.user.id,
                    user_name: content.user.urlname,
                    display_name: content.user.name,
                    avatar_url: content.user.userProfileImagePath,
                },
            })
            .collect();

        ArticleResponse {
            service_name: service_name,
            articles: articles,
            summary: "".to_string(),
        }
    }
}
