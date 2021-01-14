mod github;
mod note;
mod twitter;

use actix_cors::Cors;
use actix_web::{App, HttpServer};
use dotenv::dotenv;
use std::env;

use github::get_activities_from_github;
use note::get_activities_from_note;
use twitter::get_activities_from_twitter;

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
            .service(get_activities_from_note)
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await;
}
