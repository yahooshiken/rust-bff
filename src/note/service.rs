use super::Value;
use crate::util::http_client::create_http_client;
use actix_web::{get, HttpResponse, Responder};

#[get("/v1/activities/note")]
pub async fn get_activities_from_note() -> impl Responder {
    let url = "https://note.com/api/v2/creators/yahooshiken/contents?kind=note&page=1";
    let client = create_http_client("".to_string());

    let response = client.get(url).send().await;
    let body = response.unwrap().body().await.unwrap();
    let body_str = String::from_utf8(body.to_vec()).unwrap();
    let events: Value = serde_json::from_str(&body_str).unwrap();

    return HttpResponse::Ok()
        .content_type("application/json; charset=utf-8")
        .body(events);
}
