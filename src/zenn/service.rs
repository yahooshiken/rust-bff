use super::ZennResponse;
use actix_web::{get, HttpResponse, Responder};

use crate::util::{http_client::create_http_client, model::ArticleResponse};

#[get("/v1/activities/zenn")]
pub async fn get_activities_from_zenn() -> impl Responder {
  let url = "https://zenn.dev/api/articles?username=yahooshiken&count=500&order=latest";
  let client = create_http_client("".to_string());

  let response = client.get(url).send().await;
  let body = response.unwrap().body().await.unwrap();
  let zenn_response: ZennResponse = serde_json::from_slice(&body).unwrap();
  let article_response: ArticleResponse = ArticleResponse::from_zenn(zenn_response);
  return HttpResponse::Ok().json(article_response);
}
