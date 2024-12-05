use tracing::{info, warn};
use tracing_subscriber::prelude::*;

#[tracing::instrument]
pub fn main() {
    composition::START_TIME
        .set(std::time::Instant::now())
        .expect("could not set composition::START_TIME");

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
    let config = composition::config::Config::instance();

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
        let args = composition::config::Args::instance();
        composition::run(args.subcommand).await;
    });
}
