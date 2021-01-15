mod github;
mod note;
mod qiita;
mod twitter;
mod util {
    pub mod http_client;
    pub mod model;
}

use actix_cors::Cors;
use actix_web::{App, HttpServer};
use dotenv::dotenv;
use std::env;

use github::get_activities_from_github;
use note::get_activities_from_note;
use qiita::get_activities_from_qiita;
use twitter::get_activities_from_twitter;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let port = env::var("PORT").expect("PORT is not defined.");

    return HttpServer::new(|| {
        let allowed_origin = env::var("ALLOWED_ORIGIN").expect("ALLOWED_ORIGIN is not defined.");
        let cors = Cors::default()
            .allowed_origin(&allowed_origin)
            .allowed_methods(vec!["GET"])
            .max_age(3600);

        App::new()
            .wrap(cors)
            .service(get_activities_from_github)
            .service(get_activities_from_note)
            .service(get_activities_from_qiita)
            .service(get_activities_from_twitter)
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await;
}
