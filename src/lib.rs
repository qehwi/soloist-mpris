//! `soloist-mpris` — an [MPRIS] D-Bus server exposing Spotify's headless client
//! (`soloist`) to the desktop.
//!
//! [MPRIS]: https://specifications.freedesktop.org/mpris-spec/latest/

pub mod app;
pub mod config;
pub mod error;
pub mod mpris;
pub mod soloist;
pub mod state;
pub mod ws;
