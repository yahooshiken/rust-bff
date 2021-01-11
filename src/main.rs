use actix_web::{
    client::{Client, Connector},
    get, App, HttpResponse, HttpServer, Responder,
};
use dotenv::dotenv;
use openssl::ssl::{SslConnector, SslMethod};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
struct Actor {
    pub id: u64,
    pub login: String,
    pub url: String,
    pub avatar_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Repo {
    pub id: u64,
    pub name: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct GitHubEvent {
    pub id: String,
    pub actor: Actor,
    pub repo: Repo,
}

#[get("/v1/activities/github")]
async fn get_activities_from_github() -> impl Responder {
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

    for event in events.iter() {
        println!("Body: {:?}", event);
    }

    return HttpResponse::Ok().json(events);
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    return HttpServer::new(|| App::new().service(get_activities_from_github))
        .bind("0.0.0.0:8080")?
        .run()
        .await;
}
