use crate::{note::NoteResponse, qiita::QiitaResponse, zenn::ZennResponse, sizu::SizuResponse};

use roxmltree::Node;
use serde::{Deserialize, Serialize};
use html2text::from_read;

#[derive(Debug, Serialize, Deserialize)]
pub enum ServiceName {
    Qiita,
    Note,
    Github,
    Twitter,
    Hatena,
    Zenn,
    Sizu,
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
        let default_image_url =
            "https://cdn.qiita.com/assets/qiita-fb-41f9429f13a8c56f50dd0ab477c80d26.png";
        let articles = qiita_response
            .into_iter()
            .map(|v| Article {
                service_name: ServiceName::Qiita,
                id: v.id,
                title: v.title,
                created_at: v.created_at,
                body: v.body,
                url: v.url,
                image_url: Some(default_image_url.to_string()),
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
        let default_image_url = "https://cdn.blog.st-hatena.com/images/theme/og-image-1500.png";
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
                    image_url: Some(default_image_url.to_string()),
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

    pub fn from_zenn(zenn_response: ZennResponse) -> Self {
        let default_image_url = "https://zenn.dev/images/og-large.png";
        let articles = zenn_response
            .articles
            .into_iter()
            .map(|article| Article {
                service_name: ServiceName::Zenn,
                id: article.id.to_string(),
                title: article.title,
                created_at: article.published_at,
                image_url: Some(default_image_url.to_string()),
                url: Some(format!(
                    "https://zenn.dev/yahooshiken/articles/{}",
                    article.slug
                )), body: Some("".to_string()),
                user: User {
                    id: article.user.id,
                    user_name: article.user.username,
                    display_name: article.user.name,
                    avatar_url: article.user.avatar_small_url,
                },
                tags: Vec::new(),
            })
            .collect();

        ArticleResponse { articles }
    }

    pub fn from_sizu(sizu_response: SizuResponse) -> Self {
        let default_image_url = "	https://static.sizu.me/api/og-image/5112c30d21fe?avatarUrl=https%3A%2F%2Fr2.sizu.me%2Fusers%2F24466%2Favatar.png%3Fv%3D1704769301911&theme=user&username=yahooshiken";
        let articles = sizu_response
            .result
            .data
            .posts
            .into_iter()
            .map(|post| Article {
                service_name: ServiceName::Sizu,
                id: post.id.to_string(),
                title: Some(post.title),
                created_at: None,
                image_url: Some(default_image_url.to_string()),
                url: Some(format!("https://sizu.me/yahooshiken/posts/{}", post.slug)),
                body: Some(from_read(post.excerptHtml.as_bytes(), 80)),
                user: User {
                    id: post.userId,
                    user_name: Some("yahooshiken".to_string()),
                    display_name: Some("yahooshiken".to_string()),
                    avatar_url: Some(format!("https://r2.sizu.me/users/{}/avatar.png", post.userId)),
                },
                tags: Vec::new(),
            })
            .collect();

        ArticleResponse { articles }
    }
}
