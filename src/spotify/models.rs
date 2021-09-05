use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Tracks {
    href: String,
    total: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Image {
    width: u64,
    height: u64,
    url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Owner {
    id: String,
    display_name: String,
    href: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Playlist {
    id: String,
    name: String,
    description: String,
    public: bool,
    owner: Owner,
    images: Vec<Image>,
    tracks: Tracks,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlaylistResponse {
    href: String,
    items: Vec<Playlist>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    playlists: Vec<Playlist>,
}

impl Response {
    pub fn from_spotify_response(spotify_response: PlaylistResponse) -> Self {
        Response {
            playlists: spotify_response.items,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetTokenRequest {
    pub grant_type: String,
    pub scope: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct GetTokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u64,
    pub scope: Option<String>,
}
