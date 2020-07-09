extern crate chrono;
extern crate colorful;

use chrono::prelude::*;
use colorful::{Color, Colorful};
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;

pub fn get_timestring() -> String {
    Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}
pub fn log(logtype: &str, logmessage: &str) -> String {
    format!("{} [{}] - {}", get_timestring(), logtype, logmessage)
}

pub fn new(logfile: &str) -> Logger {
    Logger::new(logfile)
}

#[derive(Clone)]
pub struct Logger {
    pub logfile: String,
}
impl Logger {
    pub fn new(logfile: &str) -> Logger {
        Logger {
            logfile: logfile.to_owned(),
        }
    }
    pub fn important(&self, s: &str) {
        let l = log("IMPORTANT", s);
        println!("{}", l.clone().color(Color::Green));
        self.append(&l);
    }
    pub fn info(&self, s: &str) {
        let l = log("INFO", s);
        // println!("{}", l.clone().color(Color::White));
        println!("{}", l.clone());
        self.append(&l);
    }
    pub fn warn(&self, s: &str) {
        let l = log("WARN", s);
        println!("{}", l.clone().color(Color::LightYellow));
        self.append(&l);
    }
    pub fn error(&self, s: &str) {
        let l = log("ERROR", s);
        println!("{}", l.clone().color(Color::LightRed));
        self.append(&l);
    }
    pub fn append(&self, s: &str) {
        let a = || -> std::io::Result<()> {
            let mut file = OpenOptions::new()
                .create(true)
                .write(true)
                .append(true)
                .open(&self.logfile)?;
            writeln!(file, "{}", s)?;
            Ok(())
        };
        if a().is_err() {
            self.logger_error(&format!("Could not write to log file {}", self.logfile));
        }
    }
    pub fn clear(&self) {
        if std::fs::remove_file(&self.logfile).is_err() {
            self.logger_error(&format!("Could not write to log file {}", self.logfile));
        }
        self.important(&format!("Cleared log file {}", self.logfile));
    }
    pub fn read(&self) -> std::io::Result<String> {
        let mut f = File::open(&self.logfile)?;
        let mut buffer = String::new();
        f.read_to_string(&mut buffer)?;
        Ok(buffer)
    }
    pub fn logger_error(&self, error_message: &str) {
        let l = log("ERROR", error_message);
        println!("{}", l.clone().color(Color::LightRed));
    }
}
