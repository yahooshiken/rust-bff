use actix_web::{
    client::{Client, Connector},
    get, HttpRequest, HttpResponse, Responder,
};
use openssl::ssl::{SslConnector, SslMethod};
use std::env;

use crate::{
    spotify::models::{
        GetPlaylistResponse, GetTokenRequest, GetTokenResponse, GetTracksResponse,
        PlaylistResponse, TrackResponse,
    },
    util::http_client::create_http_client,
};

pub async fn get_token() -> String {
    let url = "https://accounts.spotify.com/api/token";
    let client_id = env::var("SPOTIFY_CLIENT_ID").expect("SPOTIFY_CLIENT_ID is not defined");
    let client_secret =
        env::var("SPOTIFY_CLIENT_SECRET").expect("SPOTIFY_CLIENT_SECRET is not defined");

    let builder = SslConnector::builder(SslMethod::tls()).unwrap();
    let client = Client::builder()
        .header("User-Agent", "localhost")
        .basic_auth(client_id, Some(&client_secret))
        .connector(Connector::new().ssl(builder.build()).finish())
        .finish();

    let get_token_request = GetTokenRequest {
        grant_type: "client_credentials".to_string(),
        scope: "playlist-read-private".to_string(),
    };
    let response = client.post(url).send_form(&get_token_request).await;
    let body = response.unwrap().body().await.unwrap();
    let get_token_response: GetTokenResponse = serde_json::from_slice(&body).unwrap();

    return get_token_response.access_token;
}

#[get("/v1/playlists")]
pub async fn get_playlists() -> impl Responder {
    let url = "https://api.spotify.com/v1/users/217mcmfe5wbn5pfmh2kt52zfy/playlists?limit=30";
    let token = get_token().await;
    let client = create_http_client(token);

    let response = client.get(url).send().await;
    let body = response.unwrap().body().await.unwrap();
    let playlist_response: PlaylistResponse = serde_json::from_slice(&body).unwrap();
    let res: GetPlaylistResponse = GetPlaylistResponse::from_spotify_response(playlist_response);
    return HttpResponse::Ok().json(res);
}

#[get("/v1/playlists/{playlist_id}/tracks")]
pub async fn get_tracks(req: HttpRequest) -> impl Responder {
    let playlist_id: String = req
        .match_info()
        .get("playlist_id")
        .unwrap()
        .parse()
        .unwrap();

    let url = format!(
        "https://api.spotify.com/v1/playlists/{}/tracks",
        playlist_id
    );
    let token = get_token().await;
    let client = create_http_client(token);

    let response = client.get(url).send().await;
    let body = response.unwrap().body().await.unwrap();
    let track_response: TrackResponse = serde_json::from_slice(&body).unwrap();
    let res: GetTracksResponse = GetTracksResponse::from_spotify_response(track_response);
    return HttpResponse::Ok().json(res);
}
