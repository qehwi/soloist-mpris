//! MPRIS server setup and control callbacks.

use mpris_server::{
    PlaybackStatus,
    PlaybackStatus::{Paused, Playing, Stopped},
    Player,
};
use tracing::debug;

use crate::{config::Config, error::Error, state};

impl From<state::PlaybackStatus> for PlaybackStatus {
    fn from(value: state::PlaybackStatus) -> Self {
        match value {
            state::PlaybackStatus::Playing => PlaybackStatus::Playing,
            state::PlaybackStatus::Paused => PlaybackStatus::Paused,
            state::PlaybackStatus::Stopped => PlaybackStatus::Stopped,
        }
    }
}

/// Build the MPRIS [`Player`] and register its control callbacks.
///
/// # Errors
///
/// Returns an error if the D-Bus connection or player setup fails.
//
// NOTE: the returned future is `!Send` because [`Player`] is `Rc`-backed.
#[allow(clippy::future_not_send)]
pub async fn build(config: &Config) -> Result<Player, Error> {
    let player = Player::builder(config.bus_name.as_str())
        .can_play(true)
        .can_pause(true)
        .can_go_next(true)
        .can_go_previous(true)
        .identity("Soloist")
        .build()
        .await?;

    register_controls(&player, config);

    Ok(player)
}

fn register_controls(player: &Player, config: &Config) {
    let binary = config.soloist_binary.clone();

    player.connect_play_pause({
        let binary = binary.clone();
        move |player| {
            debug!("Play/Pause called");
            let cmd = match player.playback_status() {
                Playing => "pause",
                Stopped | Paused => "play",
            };
            let binary = binary.clone();
            tokio::spawn(async move {
                let _ = tokio::process::Command::new(&binary)
                    .args(["ctl", cmd])
                    .status()
                    .await;
            });
        }
    });

    player.connect_next({
        let binary = binary.clone();
        move |_player| {
            debug!("Next called");
            let binary = binary.clone();
            tokio::spawn(async move {
                let _ = tokio::process::Command::new(&binary)
                    .args(["ctl", "next"])
                    .status()
                    .await;
            });
        }
    });

    player.connect_previous(move |_player| {
        debug!("Previous called");
        let binary = binary.clone();
        tokio::spawn(async move {
            let _ = tokio::process::Command::new(&binary)
                .args(["ctl", "prev"])
                .status()
                .await;
        });
    });
}
