use crate::config::{read_file, Args, Config};
use clap::Arg;
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::{fs::File, path::Path, path::PathBuf};
use tracing::{error, trace, warn};

const DEFAULT_SERVER_ICON: &str = "server-icon.png";

/// The main server configuration struct.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
#[serde(default)]
pub struct ServerConfig {
    #[serde(rename = "version-string")]
    pub version: String,
    pub port: u16,
    pub max_players: usize,
    pub motd: String,
    pub server_icon: PathBuf,
    #[serde(skip)]
    pub server_icon_bytes: Vec<u8>,
}
impl Default for ServerConfig {
    fn default() -> Self {
        ServerConfig {
            version: Config::get_formatted_version(crate::config::Subcommand::Server),
            port: 25565,
            max_players: 20,
            motd: "Hello world!".to_owned(),
            server_icon: PathBuf::from(DEFAULT_SERVER_ICON),
            server_icon_bytes: include_bytes!("../server-icon.png").to_vec(),
        }
    }
}
impl ServerConfig {
    pub fn instance() -> &'static Self {
        &Config::instance().server
    }
    pub fn load_args(&mut self) {
        self.server_icon = ServerArgs::instance()
            .as_ref()
            .map(|s| s.server_icon.clone())
            .unwrap_or(PathBuf::from(DEFAULT_SERVER_ICON));
        self.load_icon();
    }
    /// Load the server icon.
    pub fn load_icon(&mut self) {
        let server_icon_path = Path::new(&self.server_icon);

        if server_icon_path.exists() {
            if let Ok(server_icon_bytes) = read_file(server_icon_path) {
                self.server_icon_bytes = server_icon_bytes;
            } else {
                warn!("Could not read server icon file, using default");
            }
        } else {
            warn!(
                "Server icon file does not exist, creating {}",
                server_icon_path.to_str().unwrap_or("")
            );
            self.write_server_icon(server_icon_path);
        }
    }
    pub fn write_server_icon(&self, path: &Path) {
        trace!("ServerConfig.write_server_icon()");
        if let Ok(mut file) = File::options()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)
        {
            if file.write_all(&self.server_icon_bytes).is_ok() {
                return;
            }
        }
        error!("Could not write server icon file");
        std::process::exit(1);
    }
}

#[derive(Debug)]
pub struct ServerArgs {
    pub server_icon: PathBuf,
}
impl Default for ServerArgs {
    fn default() -> Self {
        ServerArgs {
            server_icon: PathBuf::from(DEFAULT_SERVER_ICON),
        }
    }
}
impl ServerArgs {
    pub fn instance() -> Option<&'static Self> {
        Args::instance().server.as_ref()
    }
    pub fn command() -> clap::Command {
        clap::Command::new("server")
            .about("Run composition in server mode")
            .arg(
                Arg::new("server-icon")
                    .long("server-icon")
                    .help("Server icon file path")
                    .value_hint(clap::ValueHint::FilePath)
                    .default_value(DEFAULT_SERVER_ICON),
            )
    }
    pub fn parse(m: clap::ArgMatches) -> Self {
        let mut server_args = ServerArgs::default();
        server_args.server_icon = m
            .get_one::<String>("server-icon")
            .map_or(server_args.server_icon, PathBuf::from);
        server_args
    }
}
