
use super::SizuResponse;
use actix_web::{get, HttpResponse, Responder};
use std::env;
use urlencoding::encode;

use crate::util::{http_client::create_http_client, model::ArticleResponse};

#[get("/v1/activities/sizu")]
pub async fn get_activities_from_sizu() -> impl Responder {
    let input = encode(r#"{"0":{"userId":24466,"pageNumber":1}}"#);
    let url = "https://sizu.me/api/trpc/postList.index?batch=1&input=".to_string() + &input;
    let client = create_http_client("".to_string());
    let response = client.get(url).send().await;
    let body = response.unwrap().body().await.unwrap();
    let sizu_response: Vec<SizuResponse> = serde_json::from_slice(&body).unwrap();
    let first_sizu_response = sizu_response.into_iter().nth(0);

    if first_sizu_response.is_none() {
        return HttpResponse::Ok().json(ArticleResponse { articles: vec![] });
    }

    let article_response: ArticleResponse = ArticleResponse::from_sizu(first_sizu_response.unwrap());
    return HttpResponse::Ok().json(article_response);
}
