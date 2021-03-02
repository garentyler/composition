use log::info;
use std::time::{Duration, Instant};

#[tokio::main]
pub async fn main() {
    let start_time = Instant::now();
    composition::init();
    info!("Starting server...");
    let mut server = composition::start_server().await;
    info!("Done! Start took {:?}", start_time.elapsed());

    // The main server loop.
    loop {
        server.update().await;
        std::thread::sleep(Duration::from_millis(2));
    }
}
