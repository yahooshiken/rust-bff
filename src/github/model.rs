use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Actor {
   pub id: u64,
   pub login: String,
   pub url: String,
   pub avatar_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub  struct Repo {
   pub id: u64,
   pub name: String,
   pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GitHubEvent {
   pub id: String,
   pub actor: Actor,
   pub repo: Repo,
}
