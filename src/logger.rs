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

pub struct Logger {
    pub logfile: String,
    pub info_written: u32,
    pub warn_written: u32,
    pub error_written: u32,
    pub important_written: u32,
    pub write_enabled: bool,
}
impl Logger {
    pub fn new(logfile: &str) -> Logger {
        Logger {
            logfile: logfile.to_owned(),
            info_written: 0,
            warn_written: 0,
            error_written: 0,
            important_written: 0,
            write_enabled: true,
        }
    }
    pub fn important(&mut self, s: &str) {
        let l = log("IMPORTANT", s);
        println!("{}", l.clone().color(Color::Cyan));
        self.append(&l);
        self.important_written += 1;
    }
    pub fn info(&mut self, s: &str) {
        let l = log("INFO", s);
        println!("{}", l.clone().color(Color::White));
        self.append(&l);
        self.info_written += 1;
    }
    pub fn warn(&mut self, s: &str) {
        let l = log("WARN", s);
        println!("{}", l.clone().color(Color::Yellow));
        self.append(&l);
        self.warn_written += 1;
    }
    pub fn error(&mut self, s: &str) {
        let l = log("ERROR", s);
        println!("{}", l.clone().color(Color::Red));
        self.append(&l);
        self.error_written += 1;
    }
    pub fn append(&mut self, s: &str) {
        if self.write_enabled {
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
    }
    pub fn clear(&mut self) {
        if self.write_enabled {
            if std::fs::remove_file(&self.logfile).is_err() {
                self.logger_error(&format!("Could not write to log file {}", self.logfile));
            }
        }
        self.info_written = 0;
        self.warn_written = 0;
        self.error_written = 0;
        self.important_written = 0;
        self.important(&format!("Cleared log file {}", self.logfile));
    }
    pub fn read(&self) -> std::io::Result<String> {
        let mut f = File::open(&self.logfile)?;
        let mut buffer = String::new();
        f.read_to_string(&mut buffer)?;
        Ok(buffer)
    }
    pub fn logger_error(&mut self, error_message: &str) {
        self.write_enabled = false;
        self.error(error_message);
    }
}
