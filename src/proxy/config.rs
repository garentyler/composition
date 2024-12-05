use crate::config::{Args, Config};
use clap::Arg;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::ffi::OsStr;

pub static DEFAULT_PROXY_ARGS: Lazy<ProxyArgs> = Lazy::new(ProxyArgs::default);

/// The main server configuration struct.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
#[serde(default)]
pub struct ProxyConfig {
    #[serde(rename = "version-string")]
    pub version: String,
    pub port: u16,
    pub upstream: String,
}
impl Default for ProxyConfig {
    fn default() -> Self {
        ProxyConfig {
            version: Config::get_formatted_version(crate::config::Subcommand::Proxy),
            port: 25565,
            upstream: String::new(),
        }
    }
}
impl ProxyConfig {
    pub fn instance() -> &'static Self {
        &Config::instance().proxy
    }
    pub fn load_args(&mut self) {
        self.upstream = ProxyArgs::instance()
            .as_ref()
            .map(|p| p.upstream.clone())
            .unwrap_or(DEFAULT_PROXY_ARGS.upstream.clone());
    }
}

#[derive(Debug, Default)]
pub struct ProxyArgs {
    upstream: String,
}
impl ProxyArgs {
    pub fn instance() -> Option<&'static Self> {
        Args::instance().proxy.as_ref()
    }
    pub fn command() -> clap::Command {
        clap::Command::new("proxy")
            .about("Run composition in proxy mode")
            .arg(
                Arg::new("upstream")
                    .short('u')
                    .long("upstream")
                    .help("Upstream server address")
                    .value_hint(clap::ValueHint::Hostname)
                    .default_value(OsStr::new(&DEFAULT_PROXY_ARGS.upstream)),
            )
    }
    pub fn parse(m: clap::ArgMatches) -> Self {
        let mut proxy_args = ProxyArgs::default();
        proxy_args.upstream = m
            .get_one::<String>("upstream")
            .unwrap_or(&proxy_args.upstream)
            .clone();
        proxy_args
    }
}
