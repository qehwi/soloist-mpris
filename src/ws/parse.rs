//! Parsing of WebSocket text frames into [`WsMessage`] values.

use serde_json::Error as JsonError;

use crate::ws::message::WsMessage;

/// Parse a single WebSocket text frame into a [`WsMessage`].
///
/// Unknown message types deserialize to [`WsMessage::Ignore`].
///
/// # Errors
///
/// Returns a `serde_json::Error` if the frame is not valid JSON or does not
/// match the expected shape.
///
pub fn parse_ws_line(line: &str) -> Result<WsMessage, JsonError> {
    serde_json::from_str(line)
}
