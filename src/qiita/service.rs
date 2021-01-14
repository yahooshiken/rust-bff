use super::Value;
use actix_web::{
    client::{Client, Connector},
    get, HttpResponse, Responder,
};
use openssl::ssl::{SslConnector, SslMethod};
use std::env;

#[get("/v1/activities/qiita")]
pub async fn get_activities_from_qiita() -> impl Responder {
    let url = "https://qiita.com/api/v2/authenticated_user/items?page=1&per_page=20";
    let token = env::var("QIITA_BEARER_TOKEN").expect("VAR is not defined");

    let builder = SslConnector::builder(SslMethod::tls()).unwrap();
    let client = Client::builder()
        .header("User-Agent", "localhost")
        .header("Authorization", format!("Bearer {}", token))
        .connector(Connector::new().ssl(builder.build()).finish())
        .finish();

    let response = client.get(url).send().await;
    let body = response.unwrap().body().await.unwrap();
    let body_str = String::from_utf8(body.to_vec()).unwrap();
    let events: Value = serde_json::from_str(&body_str).unwrap();

    return HttpResponse::Ok()
        .content_type("application/json; charset=utf-8")
        .body(events);
}
