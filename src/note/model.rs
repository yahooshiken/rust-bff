use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct User {
    pub id: u64,
    pub name: Option<String>,
    pub urlname: Option<String>,
    pub nickname: Option<String>,
    pub userProfileImagePath: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tag {
    pub name: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct HashTag {
    pub hashtag: Tag,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct Content {
    pub id: u64,
    // type: String, https://serde.rs/field-attrs.html#serderename--name
    pub status: Option<String>,
    pub name: Option<String>,
    pub eyecatch: Option<String>,
    pub user: User,
    pub publishAt: Option<String>,
    pub body: Option<String>,
    pub hashtags: Vec<HashTag>,
    pub noteUrl: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct Data {
    pub contents: Vec<Content>,
    pub isLastPage: bool,
    pub totalCount: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NoteResponse {
    pub data: Data,
}
