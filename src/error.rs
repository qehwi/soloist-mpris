//! Top-level error type for `soloist-mpris`.

use mpris_server::zbus;
use thiserror::Error;

use crate::config::ConfigError;

/// Errors that can occur while running the daemon.
///
/// TODO:
#[derive(Debug, Error)]
pub enum Error {
    /// Configuration could not be loaded.
    #[error("configuration error: {0}")]
    Config(#[from] ConfigError),

    /// The soloist child's stdout pipe was unavailable (internal error).
    #[error("soloist child stdout pipe is unavailable")]
    StdoutUnavailable,

    /// An I/O error occurred.
    #[error("i/o error: {0}")]
    Io(#[from] std::io::Error),

    /// The D-Bus / MPRIS layer reported an error.
    #[error("dbus error: {0}")]
    Dbus(#[from] zbus::Error),

    /// A WebSocket message could not be parsed.
    #[error("websocket parse error: {0}")]
    Parse(#[from] serde_json::Error),
}
