use clap::Arg;
use once_cell::sync::{Lazy, OnceCell};
use serde::{Deserialize, Serialize};
use std::ffi::OsStr;
use std::io::{Read, Write};
use std::{fs::File, path::Path, path::PathBuf};
use tracing::{error, trace, warn};

#[cfg(feature = "proxy")]
use crate::proxy::config::{ProxyArgs, ProxyConfig};
#[cfg(feature = "server")]
use crate::server::config::{ServerArgs, ServerConfig};

/// The globally-accessible static instance of Config.
/// On program startup, Config::load() should be called to initialize it.
pub static CONFIG: OnceCell<Config> = OnceCell::new();

/// The globablly-accessible static instance of Args.
/// On program startup, Args::load() should be called to initialize it.
pub static ARGS: OnceCell<Args> = OnceCell::new();
static DEFAULT_ARGS: Lazy<Args> = Lazy::new(Args::default);

/// Helper function to read a file from a `Path`
/// and return its bytes as a `Vec<u8>`.
#[tracing::instrument]
pub fn read_file(path: &Path) -> std::io::Result<Vec<u8>> {
    trace!("{:?}", path);
    let mut data = vec![];
    let mut file = File::open(path)?;
    file.read_to_end(&mut data)?;
    Ok(data)
}

/// The global configuration.
#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
#[serde(default)]
pub struct Config {
    #[serde(rename = "composition")]
    pub global: GlobalConfig,
    #[cfg(feature = "server")]
    pub server: ServerConfig,
    #[cfg(feature = "proxy")]
    pub proxy: ProxyConfig,
}
impl Config {
    pub fn get_formatted_version(subcommand: Subcommand) -> String {
        format!(
            "composition{} {} ({} {})",
            match subcommand {
                Subcommand::None => "",
                #[cfg(feature = "server")]
                Subcommand::Server => "",
                #[cfg(feature = "proxy")]
                Subcommand::Proxy => "-proxy",
            },
            env!("CARGO_PKG_VERSION"),
            &env!("GIT_HASH")[0..9],
            &env!("GIT_DATE")[0..10]
        )
    }
    pub fn instance() -> &'static Self {
        match CONFIG.get() {
            Some(a) => a,
            None => Self::load(),
        }
    }
    fn load() -> &'static Self {
        trace!("GlobalConfig::load()");
        let args = Args::instance();
        let mut config = Config::default();
        let config_path = Path::new(&args.config_file);

        if !config_path.exists() {
            warn!(
                "Configuration file does not exist, creating {}",
                config_path.to_str().unwrap_or("")
            );
            config.write(config_path);
        }

        if let Ok(cfg) = read_file(config_path) {
            let cfg: Result<Config, _> = toml::from_str(&String::from_utf8_lossy(&cfg));
            if let Ok(cfg) = cfg {
                config = cfg;
            } else {
                error!("Could not parse configuration file, using default");
            }
        } else {
            error!("Could not read configuration file, using default");
        }

        #[cfg(feature = "server")]
        {
            config.server.load_icon();
        }

        CONFIG.set(config).expect("could not set CONFIG");
        Self::instance()
    }
    #[tracing::instrument]
    fn write(&self, path: &Path) {
        trace!("Config.write()");
        if let Ok(mut file) = File::options()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)
        {
            if file
                .write_all(toml::to_string(&self).unwrap().as_bytes())
                .is_ok()
            {
                return;
            }
        }
        error!("Could not write configuration file");
        std::process::exit(1);
    }
}

/// The global configuration.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
#[serde(default)]
pub struct GlobalConfig {
    #[serde(skip)]
    pub version: String,
    #[serde(skip)]
    pub protocol_version: i32,
    #[serde(skip)]
    pub game_version: String,
    pub threads: Option<usize>,
}
impl Default for GlobalConfig {
    fn default() -> Self {
        GlobalConfig {
            version: Config::get_formatted_version(Subcommand::None),
            protocol_version: 762,
            game_version: "1.19.4".to_owned(),
            threads: None,
        }
    }
}
impl GlobalConfig {
    pub fn instance() -> &'static Self {
        &Config::instance().global
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub enum Subcommand {
    #[default]
    None,
    #[cfg(feature = "server")]
    Server,
    #[cfg(feature = "proxy")]
    Proxy,
}

/// All of the valid command line arguments for the composition binary.
///
/// Arguments will always override the config options specified in `composition.toml` or `Config::default()`.
#[derive(Debug)]
pub struct Args {
    config_file: PathBuf,
    pub log_level: Option<tracing::Level>,
    pub log_dir: PathBuf,
    pub subcommand: Subcommand,
    #[cfg(feature = "server")]
    pub server: Option<ServerArgs>,
    #[cfg(feature = "proxy")]
    pub proxy: Option<ProxyArgs>,
}
impl Default for Args {
    fn default() -> Self {
        Args {
            config_file: PathBuf::from("composition.toml"),
            log_level: None,
            log_dir: PathBuf::from("logs"),
            subcommand: Subcommand::None,
            #[cfg(feature = "server")]
            server: None,
            #[cfg(feature = "proxy")]
            proxy: None,
        }
    }
}
impl Args {
    pub fn instance() -> &'static Self {
        match ARGS.get() {
            Some(a) => a,
            None => Self::load(),
        }
    }
    pub fn load() -> &'static Self {
        ARGS.set(Self::parse()).expect("could not set ARGS");
        Self::instance()
    }
    #[allow(unused_mut)]
    fn command() -> clap::Command {
        let mut cmd = clap::Command::new("composition")
            .about(env!("CARGO_PKG_DESCRIPTION"))
            .disable_version_flag(true)
            .arg(
                Arg::new("version")
                    .short('V')
                    .long("version")
                    .help("Print version")
                    .global(true)
                    .action(clap::ArgAction::SetTrue),
            )
            .arg(
                Arg::new("verbose")
                    .short('v')
                    .long("verbose")
                    .help("Set log level to debug")
                    .global(true)
                    .action(clap::ArgAction::SetTrue),
            )
            .arg(
                Arg::new("config-file")
                    .short('c')
                    .long("config-file")
                    .help("Configuration file path")
                    .global(true)
                    .value_hint(clap::ValueHint::FilePath)
                    .default_value(OsStr::new(&DEFAULT_ARGS.config_file)),
            )
            .arg(
                Arg::new("log-level")
                    .short('l')
                    .long("log-level")
                    .help("Set the log level")
                    .global(true)
                    .conflicts_with("verbose")
                    .value_name("level")
                    .value_parser(["trace", "debug", "info", "warn", "error"]),
            )
            .arg(
                Arg::new("log-dir")
                    .long("log-dir")
                    .help("Set the log output directory")
                    .global(true)
                    .value_name("dir")
                    .value_hint(clap::ValueHint::DirPath)
                    .default_value(OsStr::new(&DEFAULT_ARGS.log_dir)),
            );
        #[cfg(feature = "server")]
        {
            cmd = cmd.subcommand(ServerArgs::command());
        }
        #[cfg(feature = "proxy")]
        {
            cmd = cmd.subcommand(ProxyArgs::command());
        }
        cmd
    }
    #[allow(unreachable_code)]
    fn parse() -> Self {
        let mut args = Self::default();
        let m = Self::command().get_matches();

        args.config_file = m
            .get_one::<String>("config-file")
            .map_or(args.config_file, PathBuf::from);
        args.log_dir = m
            .get_one::<String>("log-dir")
            .map_or(args.log_dir, PathBuf::from);

        if m.get_flag("verbose") {
            args.log_level = Some(tracing::Level::DEBUG);
        } else {
            args.log_level = m.get_one("log-level").map_or(args.log_level, |s: &String| {
                Some(s.parse::<tracing::Level>().unwrap())
            });
        }

        if m.get_flag("version") {
            println!("{}", GlobalConfig::default().version);
            if m.get_flag("verbose") {
                println!("release: {}", env!("CARGO_PKG_VERSION"));
                println!("commit-hash: {}", env!("GIT_HASH"));
                println!("commit-date: {}", &env!("GIT_DATE")[0..10]);
                println!("license: {}", env!("CARGO_PKG_LICENSE"));
                println!("authors: {}", env!("CARGO_PKG_AUTHORS"));
                println!("build-target: {}", env!("BUILD_TARGET"));
            }
            std::process::exit(0);
        }

        match m.subcommand() {
            #[cfg(feature = "server")]
            Some(("server", m)) => {
                args.subcommand = Subcommand::Server;
                args.server = Some(ServerArgs::parse(m.clone()))
            }
            #[cfg(feature = "proxy")]
            Some(("proxy", m)) => {
                args.subcommand = Subcommand::Proxy;
                args.proxy = Some(ProxyArgs::parse(m.clone()))
            }
            None => {
                let _ = Self::command().print_help();
                std::process::exit(0);
            }
            _ => unreachable!(),
        }

        args
    }
}
