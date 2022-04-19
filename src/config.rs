use crate::prelude::*;

pub struct Config {
    pub port: u16,
    pub max_players: usize,
    pub motd: String,
    pub favicon: String,
    pub server_string: String,
    pub log_level: log::LevelFilter,
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
            favicon: "server-icon.png".to_owned(),
            server_string: server_version.clone(),
            log_level: if cfg!(debug_assertions) {
                log::LevelFilter::Debug
            } else {
                log::LevelFilter::Info
            },
            server_version,
        }
    }
}
impl Config {
    pub fn from_file(filename: &str) -> Config {
        let read_file = |filename: &str| -> std::io::Result<String> {
            use std::{fs::File, io::prelude::*};
            let mut data = String::new();
            let mut file = File::open(filename)?;
            file.read_to_string(&mut data)?;
            Ok(data)
        };
        if let Ok(config) = read_file(filename) {
            Config::parse(&config)
        } else {
            Config::default()
        }
    }
    pub fn parse(data: &str) -> Config {
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

        if let Ok(cfg) = data.parse::<toml::Value>() {
            if let Some(&toml::Value::Integer(port)) = cfg.get("port") {
                if port < u16::MIN as i64 || port > u16::MAX as i64 {
                    warn!("Config port must be an integer in the range of {}-{}, using default port: {}", u16::MIN, u16::MAX, config.port);
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
            config.favicon = get_string(
                &cfg,
                "favicon",
                &config.favicon,
                &format!(
                    "Config favicon must be a string, using default favicon: \"{}\"",
                    config.favicon
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
        } else {
            warn!("Could not parse configuration file, using default");
            config
        }
    }
}
