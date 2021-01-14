use super::Value;
use actix_web::{get, HttpResponse, Responder};
use std::env;

use crate::util::http_client::create_http_client;

#[get("/v1/activities/qiita")]
pub async fn get_activities_from_qiita() -> impl Responder {
    let url = "https://qiita.com/api/v2/authenticated_user/items?page=1&per_page=20";
    let token = env::var("QIITA_BEARER_TOKEN").expect("VAR is not defined");
    let client = create_http_client(token);

    let response = client.get(url).send().await;
    let body = response.unwrap().body().await.unwrap();
    let body_str = String::from_utf8(body.to_vec()).unwrap();
    let events: Value = serde_json::from_str(&body_str).unwrap();

    return HttpResponse::Ok()
        .content_type("application/json; charset=utf-8")
        .body(events);
}
