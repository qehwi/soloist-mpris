use std::process::ExitCode;

use soloist_mpris::{app, config};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> ExitCode {
    init_tracing();

    let config = match config::Config::from_env() {
        Ok(config) => config,
        Err(err) => {
            tracing::error!("{err}");
            return ExitCode::FAILURE;
        }
    };

    let local = tokio::task::LocalSet::new();
    if let Err(err) = local.run_until(app::run(config)).await {
        tracing::error!("{err}");
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}

fn init_tracing() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=debug", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}
