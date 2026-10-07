# soloist-mpris

An [MPRIS] D-Bus server that exposes Spotify's headless client (`soloist`) to
the desktop, so desktop media controllers (e.g. `playerctl`, shell widgets) can
see and control playback.

> **Status:** work in progress. `playback_state` syncing and basic controls
> work; the remaining WebSocket message types (`position_sync`,
> `track_changed`, etc.) and full metadata mapping are still TODO.

[MPRIS]: https://specifications.freedesktop.org/mpris-spec/latest/

## Prerequisites

- `soloist` on `PATH`.
- A Spotify Connect device name and key.
- A D-Bus session bus.

## Configuration

Configured via environment variables:

- `SOLOIST_DEVICE_NAME` (required) — Spotify Connect device name.
- `SOLOIST_KEY` (required) — Spotify Connect key.
- `SOLOIST_BINARY` (default `soloist`) — path to the `soloist` binary.
- `SOLOIST_WS_ADDR` (default `127.0.0.1`) — address the control WebSocket binds to.
- `SOLOIST_WS_PORT` (default `3123`) — port the control WebSocket binds to.
- `SOLOIST_BUS_NAME` (default `soloist-mpris`) — MPRIS player name.

## Usage

```console
export SOLOIST_DEVICE_NAME="..."
export SOLOIST_KEY="..."
cargo run
```

Then control playback from any MPRIS client:

```console
playerctl --player soloist-mpris play-pause
```

## Architecture

```text
soloist (child process)
  ├─ stdout -> tracing logs + readiness detection
  ├─ WebSocket -> ws::parse -> PlayerState -> MPRIS Player -> D-Bus
  └─ ctl commands <- MPRIS action callbacks
```

- `src/main.rs` — entry point: tracing setup, config loading, `LocalSet` driver.
- `src/lib.rs` — crate root exposing the modules.
- `src/config.rs` — environment configuration.
- `src/state.rs` — shared playback state model.
- `src/soloist.rs` — `soloist` child-process and WebSocket lifecycle.
- `src/mpris.rs` — MPRIS server setup and control callbacks.
- `src/app.rs` — wiring/entry orchestration.
- `src/ws/` — WebSocket wire types and parsing.
- `src/error.rs` — top-level error type.

## License

Everything in this repository is licensed under the MIT license.
