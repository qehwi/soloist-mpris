//! Runtime configuration, loaded from environment variables.

use std::env;

use thiserror::Error;

/// Errors that can occur while loading configuration.
#[derive(Debug, Error)]
pub enum ConfigError {
    /// A required environment variable was not set.
    #[error("missing required environment variable `{0}`")]
    Missing(&'static str),
}

/// Runtime configuration for the daemon.
///
/// TODO: replace the hand-rolled loader with `clap` (CLI flags plus env
/// fallback) once the configuration surface stabilizes.
#[derive(Debug, Clone)]
pub struct Config {
    /// Spotify Connect device name (passed to `soloist -n`).
    pub device_name: String,
    /// Spotify Connect key (passed to `soloist -k`).
    pub key: String,
    /// Path or name of the `soloist` binary.
    pub soloist_binary: String,
    /// Address the soloist WebSocket server binds to.
    pub ws_addr: String,
    /// Port the soloist WebSocket server binds to.
    ///
    /// TODO: type this as `u16` and validate while parsing.
    pub ws_port: String,
    /// MPRIS player name — the suffix `mpris-server` appends to
    /// `org.mpris.MediaPlayer2.` to form the D-Bus well-known name.
    pub bus_name: String,
}

impl Config {
    /// Load configuration from environment variables.
    ///
    /// # Errors
    ///
    /// Returns [`ConfigError::Missing`] if a required variable is unset.
    pub fn from_env() -> Result<Self, ConfigError> {
        Ok(Self {
            device_name: required("SOLOIST_DEVICE_NAME")?,
            key: required("SOLOIST_KEY")?,
            soloist_binary: env::var("SOLOIST_BINARY").unwrap_or_else(|_| "soloist".to_owned()),
            ws_addr: env::var("SOLOIST_WS_ADDR").unwrap_or_else(|_| "127.0.0.1".to_owned()),
            ws_port: env::var("SOLOIST_WS_PORT").unwrap_or_else(|_| "3123".to_owned()),
            bus_name: env::var("SOLOIST_BUS_NAME").unwrap_or_else(|_| "soloist-mpris".to_owned()),
        })
    }
}

fn required(name: &'static str) -> Result<String, ConfigError> {
    env::var(name).map_err(|_| ConfigError::Missing(name))
}
