use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PlaylistTracks {
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
    tracks: PlaylistTracks,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlaylistResponse {
    href: String,
    items: Vec<Playlist>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetPlaylistResponse {
    playlists: Vec<Playlist>,
}

impl GetPlaylistResponse {
    pub fn from_spotify_response(spotify_response: PlaylistResponse) -> Self {
        GetPlaylistResponse {
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Artist {
    id: String,
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Album {
    id: String,
    name: String,
    album_type: String,
    artists: Vec<Artist>,
    images: Vec<Image>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Track {
    id: String,
    track_number: u64,
    name: String,
    href: String,
    album: Album,
    artists: Vec<Artist>,
    disc_number: u64,
    duration_ms: u64,
    preview_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrackItem {
    track: Track,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrackResponse {
    href: String,
    items: Vec<TrackItem>,
    limit: u64,
    total: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetTracksResponse {
    tracks: Vec<Track>,
}

impl GetTracksResponse {
    pub fn from_spotify_response(spotify_response: TrackResponse) -> Self {
        GetTracksResponse {
            tracks: spotify_response
                .items
                .into_iter()
                .map(|item| item.track)
                .collect(),
        }
    }
}
