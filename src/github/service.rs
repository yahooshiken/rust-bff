use super::GitHubEvent;
use actix_web::{get, HttpResponse, Responder};
use std::env;

use crate::util::http_client::create_http_client;

#[get("/v1/activities/github")]
pub async fn get_activities_from_github() -> impl Responder {
    let url = "https://api.github.com/users/yahooshiken/events";
    let token = env::var("GITHUB_TOKEN").expect("VAR is not defined");
    let client = create_http_client(token);

    let response = client.get(url).send().await;
    let body = response.unwrap().body().await.unwrap();
    let body_str = String::from_utf8(body.to_vec()).unwrap();
    let events: Vec<GitHubEvent> = serde_json::from_str(&body_str).unwrap();

    return HttpResponse::Ok()
        .content_type("application/json; charset=utf-8")
        .json(events);
}
