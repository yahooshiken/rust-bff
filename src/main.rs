mod github;
mod hatena;
mod note;
mod qiita;
mod spotify;
mod zenn;
mod sizu;
mod util {
    pub mod http_client;
    pub mod model;
    pub mod wake;
}

use actix_cors::Cors;
use actix_web::{App, HttpServer};
use dotenv::dotenv;
use std::env;

use github::get_activities_from_github;
use hatena::get_activities_from_hatena;
use note::get_activities_from_note;
use qiita::get_activities_from_qiita;
use spotify::{get_playlists, get_tracks};
use zenn::get_activities_from_zenn;
use sizu::get_activities_from_sizu;
use util::wake::wake_up_server;

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
            .service(wake_up_server)
            .service(get_activities_from_github)
            .service(get_activities_from_note)
            .service(get_activities_from_qiita)
            .service(get_activities_from_hatena)
            .service(get_activities_from_zenn)
            .service(get_activities_from_sizu)
            .service(get_playlists)
            .service(get_tracks)
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await;
}
