//! Serde data-transfer objects for the soloist WebSocket protocol.
//!
//! These types mirror the JSON messages emitted by soloist's WebSocket server
//! and exist only to deserialize them. Unknown fields and message types are
//! ignored rather than rejected.

use serde::Deserialize;

/// A single message from the soloist WebSocket.
///
/// TODO: model the remaining message types (`auth_state`, `track_changed`,
/// `playback_changed`, `volume_changed`, `position_sync`, `queue_changed`,
/// `device_changed`) and replace [`WsMessage::Ignore`] with explicit variants.
#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum WsMessage {
    /// A full playback-state snapshot.
    PlaybackState {
        status: PlaybackStateStatus,
        item: PlaybackStateItem,
        position: PlaybackStatePosition,
    },

    /// Any message type not explicitly modeled above.
    #[serde(other)]
    Ignore,
}

/// Playback status reported by soloist.
///
/// TODO: decide how unknown statuses should be handled (e.g. an
/// `#[serde(other)]` fallback variant) once state mapping is implemented.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlaybackStateStatus {
    Playing,
    Paused,
    Buffering,
}

/// The item currently selected for playback.
#[derive(Debug, Deserialize)]
pub struct PlaybackStateItem {
    pub decorations: Decorations,
}

/// Decorations attached to a track entity.
///
/// TODO: also model `visual_identity` (cover art) and `parent` (album) when
/// implementing metadata mapping.
#[derive(Debug, Deserialize)]
pub struct Decorations {
    pub identity: DecorationsIdentity,
    pub creators: Vec<ItemCreator>,
    pub playback: ItemPlayback,
}

/// Human-readable identity of an entity.
#[derive(Debug, Deserialize)]
pub struct DecorationsIdentity {
    pub name: String,
}

/// A creator (artist) of a track.
#[derive(Debug, Deserialize)]
pub struct ItemCreator {
    pub entity: ItemCreatorEntity,
}

/// The entity behind a creator entry.
#[derive(Debug, Deserialize)]
pub struct ItemCreatorEntity {
    pub decorations: CreatorEntityDecorations,
}

/// Decorations on a creator entity.
#[derive(Debug, Deserialize)]
pub struct CreatorEntityDecorations {
    pub identity: DecorationsIdentity,
}

/// Playback information attached to a track.
#[derive(Debug, Deserialize)]
pub struct ItemPlayback {
    pub duration_ms: i64,
}

/// The current playback position.
#[derive(Debug, Deserialize)]
pub struct PlaybackStatePosition {
    pub position_ms: i64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserializes_correctly() {
        let line = r#"{"type":"playback_state","status":"paused","item":{"uri":"spotify:track:aaaaaaaaaaaaaaaaaaaaaa","entity_type":"track","decorations":{"identity":{"name":"Track Name"},"visual_identity":{"cover":[{"url":"https://example.com/cover-small.jpg","size":"small"},{"url":"https://example.com/cover-default.jpg","size":"default"},{"url":"https://example.com/cover-large.jpg","size":"large"},{"url":"https://example.com/cover-xlarge.jpg","size":"xlarge"}]},"parent":{"entity":{"uri":"spotify:album:bbbbbbbbbbbbbbbbbbbbbb","entity_type":"album","decorations":{"identity":{"name":"Album Name"}}}},"creators":[{"entity":{"uri":"spotify:artist:cccccccccccccccccccc","entity_type":"artist","decorations":{"identity":{"name":"Artist Name"}}}}],"playback":{"duration_ms":192000,"content_ratings":[]}}},"context":{"uri":"spotify:playlist:dddddddddddddddddddddd","entity_type":"playlist","decorations":{"identity":{"name":"Playlist Name"}}},"position":{"position_ms":2716,"timestamp_ms":1700000000000,"speed":0},"volume":40,"is_active":false,"options":{"shuffle":true,"repeat":"off","playback_speed":1,"modes":{"context_enhancement":"NONE"}},"available_actions":{"add_to_queue":{},"play":{},"seek":{},"seek_backward":{"step_ms":15000},"seek_forward":{"step_ms":15000},"set_repeat":{},"shuffle":{},"skip_next":{},"skip_prev":{}}}
"#;

        serde_json::from_str::<WsMessage>(line).unwrap();
    }
}
