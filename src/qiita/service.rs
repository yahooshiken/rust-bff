use super::QiitaResponse;
use actix_web::{get, HttpResponse, Responder};
use std::env;

use crate::util::{http_client::create_http_client, model::ArticleResponse};

#[get("/v1/activities/qiita")]
pub async fn get_activities_from_qiita() -> impl Responder {
    let url = "https://qiita.com/api/v2/authenticated_user/items?page=1&per_page=20";
    let token = env::var("QIITA_BEARER_TOKEN").expect("VAR is not defined");
    let client = create_http_client(token);

    let response = client.get(url).send().await;
    let body = response.unwrap().body().await.unwrap();
    let qiita_response: Vec<QiitaResponse> = serde_json::from_slice(&body).unwrap();
    let article_response: ArticleResponse = ArticleResponse::from_qiita(qiita_response);

    return HttpResponse::Ok().json(article_response);
}
