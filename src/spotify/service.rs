use actix_web::{get, HttpResponse, Responder};
use std::env;

use crate::{
    spotify::models::{PlaylistResponse, Response},
    util::http_client::create_http_client,
};

#[get("/v1/playlists")]
pub async fn get_playlists() -> impl Responder {
    let url = "https://api.spotify.com/v1/users/217mcmfe5wbn5pfmh2kt52zfy/playlists?limit=10";
    let key = "SPOTIFY_ACCESS_TOKEN";
    let token = env::var(key).expect(&format!("{} is not defined", key));
    let client = create_http_client(token);

    let response = client.get(url).send().await;
    let body = response.unwrap().body().await.unwrap();
    let playlist_response: PlaylistResponse = serde_json::from_slice(&body).unwrap();
    let res: Response = Response::from_spotify_response(playlist_response);
    return HttpResponse::Ok().json(res);
}
