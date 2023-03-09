use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct User {
  pub id: u64,
  pub name: Option<String>,
  pub username: Option<String>,
  pub avatar_small_url: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Article {
  pub id: u64,
  pub slug: String,
  pub title: Option<String>,
  pub published: bool,
  pub published_at: Option<String>,
  pub user: User,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ZennResponse {
  pub articles: Vec<Article>,
}
