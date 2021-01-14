use super::Value;
use actix_web::{
    client::{Client, Connector},
    get, HttpResponse, Responder,
};
use openssl::ssl::{SslConnector, SslMethod};

#[get("/v1/activities/note")]
pub async fn get_activities_from_note() -> impl Responder {
    let url = "https://note.com/api/v2/creators/yahooshiken/contents?kind=note&page=1";
    let builder = SslConnector::builder(SslMethod::tls()).unwrap();
    let client = Client::builder()
        .header("User-Agent", "localhost")
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
