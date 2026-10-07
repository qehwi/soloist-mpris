//! Application wiring: ties configuration, the MPRIS server, and the soloist
//! child process together.

use std::sync::Arc;

use mpris_server::{Metadata, Player, Time};
use tokio::sync::watch;
use tracing::{debug, info};

use crate::{config::Config, error::Error, mpris, soloist, state::PlayerState};

/// Run the daemon until it is shut down.
///
/// # Errors
///
/// Returns an error if MPRIS setup or soloist startup fails.
//
// NOTE: `mpris_server::Player` is `!Send` (it is `Rc`-backed), so the player
// and its updater must stay on the `LocalSet` driven by `main`.
#[allow(clippy::future_not_send)]
pub async fn run(config: Config) -> Result<(), Error> {
    let (watch_tx, watch_rx) = watch::channel(PlayerState::default());

    let soloist_rdy = Arc::new(tokio::sync::Notify::new());
    let soloist_rdy_clone = soloist_rdy.clone();

    let mut child = soloist::spawn(&config)?;
    let stdout = child.stdout.take().ok_or(Error::StdoutUnavailable)?;
    soloist::spawn_stdout_logger(stdout, soloist_rdy);
    soloist::spawn_ws_reader(&config, soloist_rdy_clone, watch_tx);

    let player = mpris::build(&config).await?;
    info!("MPRIS server initialized");

    tokio::task::spawn_local(async move {
        tokio::select! {
            result = player.run() => {
                info!("MPRIS server stopped: {result:?}");
            }
            () = update_player_loop(&player, watch_rx) => {
                debug!("state updater stopped");
            }
        }
    });

    // TODO: implement graceful shutdown (SIGINT/SIGTERM): stop the `soloist`
    // child (`child` above) and exit instead of pending forever.
    std::future::pending::<()>().await;

    Ok(())
}

/// Apply `watch`-channel updates to the MPRIS player.
#[allow(clippy::future_not_send)] // intentionally `!Send`: runs on the `LocalSet`
async fn update_player_loop(player: &Player, mut rx: watch::Receiver<PlayerState>) {
    loop {
        let val = rx.borrow_and_update();
        debug!("applying new state to mpris player {val:#?}");

        let _ = player.set_playback_status(val.status.into()).await;
        let _ = player.set_metadata(Metadata::from(val.track.clone())).await;
        player.set_position(Time::from_millis(
            val.position.as_millis().try_into().unwrap_or(0),
        ));
        let _ = player.set_shuffle(val.shuffle).await;

        drop(val);

        if rx.changed().await.is_err() {
            debug!("watch_rx dropped");
            break;
        }
    }
}
