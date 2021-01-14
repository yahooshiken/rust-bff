use super::GitHubEvent;
use actix_web::{
    client::{Client, Connector},
    get, HttpResponse, Responder,
};
use openssl::ssl::{SslConnector, SslMethod};
use std::env;

#[get("/v1/activities/github")]
pub async fn get_activities_from_github() -> impl Responder {
    let url = "https://api.github.com/users/yahooshiken/events";
    let token = env::var("GITHUB_TOKEN").expect("VAR is not defined");

    let builder = SslConnector::builder(SslMethod::tls()).unwrap();
    let client = Client::builder()
        .header("User-Agent", "localhost")
        .header("Authorization", format!("token: {}", token))
        .connector(Connector::new().ssl(builder.build()).finish())
        .finish();

    let response = client.get(url).send().await;
    let body = response.unwrap().body().await.unwrap();
    let body_str = String::from_utf8(body.to_vec()).unwrap();
    let events: Vec<GitHubEvent> = serde_json::from_str(&body_str).unwrap();

    return HttpResponse::Ok().json(events);
}
