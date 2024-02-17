use std::env;

use crate::util::model::ArticleResponse;
use awc::Client;
use actix_web::{
    get, HttpResponse, Responder,
};
use roxmltree::Node;

#[get("/v1/activities/hatena")]
pub async fn get_activities_from_hatena() -> impl Responder {
    let url =
        "https://blog.hatena.ne.jp/hoshimure-47/hoshimureinforcement.hatenablog.com/atom/entry";

    let basic_username = "hoshimure-47";
    let basic_password = env::var("HATENA_BASIC_PASSWORD").expect("VAR is not defined");

    let client = Client::builder()
        .add_default_header(("User-Agent", "localhost"))
        .basic_auth(basic_username, Some(&basic_password))
        .finish();

    let response = client.get(url).send().await;
    let body = response.unwrap().body().limit(512 * 1024).await.unwrap();
    let body = std::str::from_utf8(&body).unwrap();

    let doc = roxmltree::Document::parse(body).unwrap();
    let entries: Vec<Node> = doc
        .root()
        .descendants()
        .filter(|n| n.tag_name().name() == "entry")
        .filter(|n| {
            n.descendants()
                .find(|x| x.tag_name().name() == "draft")
                .unwrap()
                .text()
                .unwrap()
                == "no"
        })
        .collect();
    let article_response = ArticleResponse::from_hatena(entries);

    return HttpResponse::Ok().json(article_response);
}
