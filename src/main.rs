use std::time::Instant;
use tracing::{info, instrument, warn};
use tracing_subscriber::prelude::*;

#[instrument]
pub fn main() {
    let start_time = Instant::now();

    // Set up logging.
    let file_writer =
        tracing_appender::rolling::daily(&composition::config::Args::instance().log_dir, "log");
    let (file_writer, _guard) = tracing_appender::non_blocking(file_writer);

    tracing_subscriber::registry()
        .with(tracing_subscriber::filter::LevelFilter::from_level(
            composition::config::Args::instance()
                .log_level
                .unwrap_or(if cfg!(debug_assertions) {
                    tracing::Level::DEBUG
                } else {
                    tracing::Level::INFO
                }),
        ))
        .with(
            tracing_subscriber::fmt::layer()
                .compact()
                .with_ansi(false)
                .with_writer(file_writer),
        )
        .with(
            tracing_subscriber::fmt::layer()
                .compact()
                .with_writer(std::io::stdout),
        )
        .init();

    // Load the config.
    let config = composition::config::Config::load();

    match config.server_threads {
        Some(1) => {
            warn!("Running on only one thread");
            tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
        }
        Some(n) => {
            info!("Running on {} threads", n);
            tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .worker_threads(n)
                .build()
        }
        None => tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build(),
    }
    .unwrap()
    .block_on(async move {
        info!("Starting {} on port {}", config.server_version, config.port);
        let (mut server, running) = composition::start_server(start_time).await;
        info!("Done! Start took {:?}", start_time.elapsed());

        // The main server loop.
        loop {
            tokio::select! {
                _ = running.cancelled() => {
                    break;
                }
                _ = server.update() => {}
            }
        }

        let _ = tokio::time::timeout(std::time::Duration::from_secs(10), server.shutdown()).await;
    });
}
