mod github;

use actix_cors::Cors;
use actix_web::{
    client::{Client, Connector},
    get, App, HttpResponse, HttpServer, Responder,
};
use dotenv::dotenv;
use openssl::ssl::{SslConnector, SslMethod};
use serde_json::Value;
use std::env;

use github::{get_activities_from_github};

#[get("/v1/activities/twitter")]
async fn get_activities_from_twitter() -> impl Responder {
    let url = "https://api.twitter.com/2/users/1084336920/tweets?tweet.fields=created_at&expansions=author_id&user.fields=created_at&max_results=10";
    let token = env::var("TWITTER_BEARER_TOKEN").expect("VAR is not defined");

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

    return HttpResponse::Ok().body(events);
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let port = env::var("PORT").expect("PORT is not defined.");

    return HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin("http://localhost:8000")
            .allowed_methods(vec!["GET"])
            .max_age(3600);

        App::new()
            .wrap(cors)
            .service(get_activities_from_twitter)
            .service(get_activities_from_github)
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await;
}
