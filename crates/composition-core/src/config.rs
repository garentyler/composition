use crate::prelude::*;
use std::{fs::File, path::Path};

fn read_file(path: &Path) -> std::io::Result<Vec<u8>> {
    let mut data = vec![];
    let mut file = File::open(path)?;
    file.read_to_end(&mut data)?;
    Ok(data)
}

pub struct Config {
    pub port: u16,
    pub max_players: usize,
    pub motd: String,
    pub server_icon: String,
    pub server_icon_bytes: Vec<u8>,
    pub server_string: String,
    pub log_level: log::LevelFilter,
    pub protocol_version: i32,
    pub game_version: String,
    pub server_version: String,
}
impl Default for Config {
    fn default() -> Self {
        let server_version = format!(
            "composition/{} ({})",
            env!("CARGO_PKG_VERSION"),
            env!("GIT_HASH").substring(0, 8)
        );
        Config {
            port: 25565,
            max_players: 20,
            motd: "Hello world!".to_owned(),
            server_icon: "server-icon.png".to_owned(),
            server_icon_bytes: include_bytes!("./server-icon.png").to_vec(),
            server_string: server_version.clone(),
            log_level: if cfg!(debug_assertions) {
                log::LevelFilter::Debug
            } else {
                log::LevelFilter::Info
            },
            protocol_version: 756,
            game_version: "1.18.1".to_owned(),
            server_version,
        }
    }
}
impl Config {
    pub fn from_toml(cfg: toml::Value) -> Config {
        let mut config = Config::default();

        let get_string = |cfg: &toml::Value, field: &str, default: &str, error: &str| -> String {
            if let Some(s) = cfg.get(field) {
                if let Some(s) = s.as_str() {
                    return s.to_owned();
                } else {
                    warn!("{}", error);
                }
            } else {
                warn!("{}", error);
            }
            default.to_owned()
        };

        if let Some(&toml::Value::Integer(port)) = cfg.get("port") {
            if port < u16::MIN as i64 || port > u16::MAX as i64 {
                warn!(
                    "Config port must be an integer in the range of {}-{}, using default port: {}",
                    u16::MIN,
                    u16::MAX,
                    config.port
                );
            } else {
                config.port = port as u16;
            }
        } else {
            warn!(
                "Config port must be an integer in the range of {}-{}, using default port: {}",
                u16::MIN,
                u16::MAX,
                config.port
            );
        }

        if let Some(&toml::Value::Integer(max_players)) = cfg.get("max_players") {
            if max_players < 0 {
                warn!("Config max_players must be an integer in the range of {}-{}, using default max_players: {}", usize::MIN, usize::MAX, config.max_players);
            } else {
                config.max_players = max_players as usize;
            }
        } else {
            warn!("Config max_players must be an integer in the range of {}-{}, using default max_players: {}", usize::MIN, usize::MAX, config.max_players);
        }

        config.motd = get_string(
            &cfg,
            "motd",
            &config.motd,
            &format!(
                "Config motd must be a string, using default motd: \"{}\"",
                config.motd
            ),
        );
        config.game_version = get_string(
            &cfg,
            "ping_game_version",
            &config.game_version,
            &format!(
                "Config ping_game_version must be a string, using default ping_game_version: \"{}\"",
                config.game_version
            ),
        );
        config.server_icon = get_string(
            &cfg,
            "server_icon",
            &config.server_icon,
            &format!(
                "Config server_icon must be a string, using default server_icon: \"{}\"",
                config.server_icon
            ),
        );
        let default_log_level = format!("{}", config.log_level).to_ascii_lowercase();
        config.log_level = match &get_string(
            &cfg,
            "log_level",
            &default_log_level,
            &format!(
                "Config log_level must be a string, using default log_level: {}",
                default_log_level
            ),
        )[..]
        {
            "off" => log::LevelFilter::Off,
            "error" => log::LevelFilter::Error,
            "warn" => log::LevelFilter::Warn,
            "info" => log::LevelFilter::Info,
            "debug" => log::LevelFilter::Debug,
            "trace" => log::LevelFilter::Trace,
            _ => {
                warn!("Config log_level must be one of the predefined levels: off, error, warn, info, debug, trace");
                config.log_level
            }
        };
        config
    }
    pub fn load() -> Config {
        let mut config = Config::default();

        // Load the config
        let config_path = Path::new("composition.toml");
        if config_path.exists() {
            if let Ok(cfg) = read_file(config_path) {
                let cfg = String::from_utf8_lossy(&cfg);
                if let Ok(cfg) = cfg.parse::<toml::Value>() {
                    config = Config::from_toml(cfg);
                } else {
                    error!("Could not parse configuration file");
                    std::process::exit(1);
                }
            } else {
                warn!(
                    "Could not read configuration file, creating {}",
                    config_path.to_str().unwrap_or("")
                );
                if config.write(config_path).is_err() {
                    error!("Could not write configuration file");
                    std::process::exit(1);
                }
            }
        } else {
            warn!(
                "Configuration file does not exist, creating {}",
                config_path.to_str().unwrap_or("")
            );
            if config.write(config_path).is_err() {
                error!("Could not write configuration file");
                std::process::exit(1);
            }
        }

        // Load the server icon
        let server_icon_path = Path::new(&config.server_icon);
        if server_icon_path.exists() {
            if let Ok(server_icon_bytes) = read_file(server_icon_path) {
                config.server_icon_bytes = server_icon_bytes;
            } else {
                warn!(
                    "Could not read server icon file, creating {}",
                    server_icon_path.to_str().unwrap_or("")
                );
                if config.write_server_icon(server_icon_path).is_err() {
                    error!("Could not write server icon file");
                    std::process::exit(1);
                }
            }
        } else {
            warn!(
                "Server icon file does not exist, creating {}",
                server_icon_path.to_str().unwrap_or("")
            );
            if config.write_server_icon(server_icon_path).is_err() {
                error!("Could not write server icon file");
                std::process::exit(1);
            }
        }

        config
    }
    pub fn write(&self, path: &Path) -> std::io::Result<()> {
        use toml::{map::Map, Value};

        let config = Value::Table({
            let mut m = Map::new();
            m.insert(
                "server_icon".to_owned(),
                Value::String(self.server_icon.clone()),
            );
            m.insert(
                "log_level".to_owned(),
                Value::String(format!("{}", self.log_level).to_ascii_lowercase()),
            );
            m.insert("max_players".to_owned(), Value::Integer(20));
            m.insert("motd".to_owned(), Value::String(self.motd.clone()));
            m.insert(
                "ping_game_version".to_owned(),
                Value::String(self.game_version.clone()),
            );
            m.insert("port".to_owned(), Value::Integer(25565));
            m
        });
        if path.exists() {
            std::fs::remove_file(path)?;
        }
        let mut file = File::create(path)?;
        file.write_all(&toml::ser::to_vec(&config).unwrap())?;
        Ok(())
    }
    pub fn write_server_icon(&self, path: &Path) -> std::io::Result<()> {
        if path.exists() {
            std::fs::remove_file(path)?;
        }
        let mut file = File::create(path)?;
        file.write_all(&self.server_icon_bytes)?;
        Ok(())
    }
}
