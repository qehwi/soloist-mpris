//! Shared playback state — the single source of truth between the WebSocket
//! reader and the MPRIS layer.

use std::time::Duration;

use crate::ws::message::WsMessage;

/// Playback status, mirroring MPRIS `PlaybackStatus`.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum PlaybackStatus {
    Playing,
    Paused,
    #[default]
    Stopped,
}

/// Repeat mode, mirroring MPRIS `LoopStatus`.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Repeat {
    #[default]
    None,
    Track,
    Playlist,
}

/// Metadata for the currently selected track.
#[derive(Debug, Clone, Default)]
pub struct Track {
    /// Track title.
    pub title: String,
    /// Artist names.
    pub artists: Vec<String>,
    /// Album name.
    pub album: String,
    /// URL of the largest available cover image.
    pub art_url: Option<String>,
    /// Spotify URI (e.g. `spotify:track:...`).
    pub uri: String,
    /// Track length, if known.
    pub duration: Option<Duration>,
}

impl From<Track> for mpris_server::Metadata {
    fn from(value: Track) -> Self {
        let Track {
            title,
            artists,
            album,
            ..
        } = value;
        mpris_server::Metadata::builder()
            .title(title)
            .artist(artists)
            .album(album)
            //TODO:
            //.art_url(art_url)
            //.length(duration.and_then(|dur| Some(Time::from_millis(dur.as_millis()))))
            .build()
    }
}

/// The full, mutable player state.
///
#[derive(Debug, Clone, Default)]
pub struct PlayerState {
    pub status: PlaybackStatus,
    pub track: Track,
    pub position: Duration,
    pub volume: f64,
    pub shuffle: bool,
    pub repeat: Repeat,
}

impl PlayerState {
    pub fn apply(&mut self, ws_message: WsMessage) {
        match ws_message {
            WsMessage::PlaybackState {
                status,
                item,
                position,
            } => {
                self.status = match status {
                    crate::ws::message::PlaybackStateStatus::Playing => PlaybackStatus::Playing,
                    crate::ws::message::PlaybackStateStatus::Paused => PlaybackStatus::Paused,
                    crate::ws::message::PlaybackStateStatus::Buffering => self.status,
                };
                self.track = Track {
                    title: item.decorations.identity.name,
                    artists: item
                        .decorations
                        .creators
                        .iter()
                        .map(|c| c.entity.decorations.identity.name.clone())
                        .collect(),
                    //TODO:depends on full ws message impl
                    album: self.track.album.clone(),
                    art_url: self.track.art_url.clone(),
                    uri: self.track.uri.clone(),
                    duration: Some(Duration::from_millis(
                        item.decorations.playback.duration_ms.cast_unsigned(),
                    )),
                };
                self.position = Duration::from_millis(position.position_ms.cast_unsigned());
            }
            WsMessage::Ignore => (),
        }
    }
}
