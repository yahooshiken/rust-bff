use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Time {
  pub short: String,
  pub formatted: String,
  pub y: String,
  pub m: String,
  pub d: String,
  pub iso: String,
  pub time: u64,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Post {
  pub id: u64,
  pub userId: u64,
  pub slug: String,
  pub visibility: String,
  pub title: String,
  pub createdAt: Time,
  pub bodyUpdatedAt: Time,
  pub excerptHtml: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
  pub posts: Vec<Post>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Result {
  pub data: Data,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SizuResponse {
  pub result: Result,
}
