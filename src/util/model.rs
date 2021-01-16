use crate::{note::NoteResponse, qiita::QiitaResponse};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum ServiceName {
    Qiita,
    Note,
    Github,
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
    service_name: ServiceName,
    id: String,
    title: Option<String>,
    created_at: Option<String>,
    body: Option<String>,
    image_url: Option<String>,
    tags: Vec<String>,
    user: User,
    url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArticleResponse {
    pub articles: Vec<Article>,
}

impl ArticleResponse {
    pub fn from_note(note_response: NoteResponse) -> Self {
        let articles = note_response
            .data
            .contents
            .into_iter()
            .map(|content| Article {
                service_name: ServiceName::Note,
                id: content.id.to_string(),
                title: content.name,
                created_at: content.publishAt,
                body: content.body,
                url: content.noteUrl,
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

        ArticleResponse { articles }
    }

    pub fn from_qiita(qiita_response: Vec<QiitaResponse>) -> Self {
        let articles = qiita_response
            .into_iter()
            .map(|v| Article {
                service_name: ServiceName::Qiita,
                id: v.id,
                title: v.title,
                created_at: v.created_at,
                body: v.body,
                url: v.url,
                image_url: None,
                tags: v.tags.into_iter().map(|tag| tag.name).collect(),
                user: User {
                    id: v.user.permanent_id,
                    user_name: v.user.id,
                    display_name: None,
                    avatar_url: v.user.profile_image_url,
                },
            })
            .collect();

        ArticleResponse { articles }
    }
}
