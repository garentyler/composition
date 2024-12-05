use crate::config::{Args, Config};
use serde::{Deserialize, Serialize};

/// The main server configuration struct.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
#[serde(default)]
pub struct ProxyConfig {
    #[serde(rename = "version-string")]
    pub version: String,
    pub port: u16,
    pub upstream_address: String,
}
impl Default for ProxyConfig {
    fn default() -> Self {
        ProxyConfig {
            version: Config::get_formatted_version(crate::config::Subcommand::Proxy),
            port: 25565,
            upstream_address: String::new(),
        }
    }
}
impl ProxyConfig {
    pub fn instance() -> &'static Self {
        &Config::instance().proxy
    }
}

#[derive(Debug)]
pub struct ProxyArgs {}
impl Default for ProxyArgs {
    fn default() -> Self {
        ProxyArgs {}
    }
}
impl ProxyArgs {
    pub fn instance() -> Option<&'static Self> {
        Args::instance().proxy.as_ref()
    }
    pub fn command() -> clap::Command {
        clap::Command::new("proxy").about("Run composition in proxy mode")
    }
    pub fn parse(_: clap::ArgMatches) -> Self {
        ProxyArgs::default()
    }
}
