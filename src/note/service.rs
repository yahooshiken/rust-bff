use super::NoteResponse;
use crate::util::http_client::create_http_client;
use crate::util::model::ArticleResponse;
use actix_web::{get, HttpResponse, Responder};

#[get("/v1/activities/note")]
pub async fn get_activities_from_note() -> impl Responder {
    let url = "https://note.com/api/v2/creators/yahooshiken/contents?kind=note&page=1";
    let client = create_http_client("".to_string());

    let response = client.get(url).send().await;
    let body = response.unwrap().body().await.unwrap();
    let note_response: NoteResponse = serde_json::from_slice(&body).unwrap();
    let article_response: ArticleResponse = ArticleResponse::from_note(note_response);

    return HttpResponse::Ok().json(article_response);
}
