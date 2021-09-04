use serde::{Deserialize, Serialize};

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
