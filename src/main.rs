use log::*;
use std::sync::mpsc::TryRecvError;
use std::time::Duration;

#[tokio::main]
pub async fn main() {
    let ctrlc_rx = composition::init();
    info!("Starting server...");
    let mut server = composition::start_server().await;
    info!("Done! Start took {:?}", composition::START_TIME.elapsed());

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
