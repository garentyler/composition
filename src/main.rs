#![allow(non_snake_case)]

pub mod logger;

fn main() {
    let mut log = logger::new("log.txt");
    log.clear();
    log.important("This is important information");
    log.info("This is information");
    log.warn("This is a warning");
    log.error("This is an error");
}
