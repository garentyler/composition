use log::info;
use std::sync::mpsc::TryRecvError;
use std::time::Duration;

#[tokio::main]
pub async fn main() {
    let ctrlc_rx = composition_core::init();
    info!(
        "Starting {} on port {}",
        composition_core::CONFIG.server_version,
        composition_core::CONFIG.port
    );
    let mut server = composition_core::start_server().await;
    info!(
        "Done! Start took {:?}",
        composition_core::START_TIME.elapsed()
    );

    // The main server loop.
    loop {
        match ctrlc_rx.try_recv() {
            Ok(_) => {
                let _ = server.shutdown().await;
                break; // Exit the loop.
            }
            Err(TryRecvError::Empty) => {} // Doesn't matter if there's nothing for us.
            Err(TryRecvError::Disconnected) => panic!("Ctrl-C sender disconnected"),
        }
        server.update().await.unwrap();
        std::thread::sleep(Duration::from_millis(2));
    }
}
