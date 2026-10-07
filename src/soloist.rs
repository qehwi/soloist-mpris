//! Management of the `soloist` child process and its WebSocket stream.

use std::{process::Stdio, sync::Arc};

use futures_util::StreamExt;
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    process::{Child, ChildStdout, Command},
    sync::{Notify, watch},
};
use tokio_tungstenite::{connect_async, tungstenite};
use tracing::{debug, error, info};

use crate::{config::Config, error::Error, state::PlayerState, ws::parse::parse_ws_line};

/// Spawn the `soloist` child process and expose its control WebSocket.
///
/// # Errors
///
/// Returns an error if the child process cannot be spawned.
pub fn spawn(config: &Config) -> Result<Child, Error> {
    let ws_bind = format!("{}:{}", config.ws_addr, config.ws_port);

    let child = Command::new(config.soloist_binary.as_str())
        .args([
            "-n",
            config.device_name.as_str(),
            "-k",
            config.key.as_str(),
            "-w",
            ws_bind.as_str(),
        ])
        .stdout(Stdio::piped())
        .spawn()?;

    info!("Soloist child spawned");
    Ok(child)
}

/// Forward the child's stdout to the tracing log.
///
pub fn spawn_stdout_logger(stdout: ChildStdout, soloist_rdy: Arc<Notify>) {
    tokio::spawn(async move {
        let mut lines = BufReader::new(stdout).lines();
        while let Ok(Some(line)) = lines.next_line().await {
            info!("Soloist: {line}");
            if line.contains("running") {
                soloist_rdy.notify_one();
            }
        }
    });
}

/// Connect to the child's WebSocket and log incoming events.
///
pub fn spawn_ws_reader(
    config: &Config,
    soloist_rdy: Arc<Notify>,
    mpris_watcher: watch::Sender<PlayerState>,
) {
    let ws_url = format!("ws://{}:{}", config.ws_addr, config.ws_port);

    tokio::spawn(async move {
        soloist_rdy.notified().await;

        let Ok((mut ws, _)) = connect_async(&ws_url).await else {
            error!("Failed to establish ws connection");
            return;
        };

        let mut state = PlayerState {
            ..Default::default()
        };
        while let Some(input) = ws.next().await {
            match input {
                Ok(msg) => {
                    if let tungstenite::Message::Text(t) = msg {
                        match parse_ws_line(t.as_str()) {
                            Ok(ws_msg) => {
                                debug!("Parse success {ws_msg:#?}");
                                state.apply(ws_msg);
                                mpris_watcher.send_replace(state.clone());
                            }
                            Err(e) => error!("parse error {e}"),
                        }
                    }
                }
                Err(e) => error!("{e}"),
            }
        }
    });
}
