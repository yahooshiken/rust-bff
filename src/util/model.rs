use crate::{note::NoteResponse, qiita::QiitaResponse};

use roxmltree::Node;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum ServiceName {
    Qiita,
    Note,
    Github,
    Twitter,
    Hatena,
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

    pub fn from_hatena(hatena_response: Vec<Node>) -> Self {
        let articles = hatena_response
            .into_iter()
            .map(|n| {
                let id = n
                    .descendants()
                    .find(|x| x.tag_name().name() == "id")
                    .unwrap()
                    .text();
                let title = n
                    .descendants()
                    .find(|x| x.tag_name().name() == "title")
                    .unwrap()
                    .text();

                let created_at = n
                    .descendants()
                    .find(|x| x.tag_name().name() == "published")
                    .unwrap()
                    .text();

                let body = n
                    .descendants()
                    .find(|x| x.tag_name().name() == "summary")
                    .unwrap()
                    .text();

                let url = n
                    .descendants()
                    .find(|x| x.attribute("rel").unwrap_or_default() == "alternate")
                    .unwrap()
                    .attribute("href")
                    .unwrap();

                let author = n
                    .descendants()
                    .find(|x| x.tag_name().name() == "author")
                    .unwrap()
                    .descendants()
                    .find(|x| x.tag_name().name() == "name")
                    .unwrap()
                    .text();

                Article {
                    service_name: ServiceName::Hatena,
                    id: id.unwrap_or("").to_string(),
                    title: title.map(|s| s.to_string()),
                    created_at: created_at.map(|s| s.to_string()),
                    body: body.map(|s| s.to_string()),
                    url: Some(url.to_string()),
                    image_url: None,
                    tags: vec![],
                    user: User {
                        id: 1,
                        display_name: author.map(|s| s.to_string()),
                        user_name: author.map(|s| s.to_string()),
                        avatar_url: None,
                    },
                }
            })
            .collect();

        ArticleResponse { articles }
    }
}