use crate::config::{Args, Config};
use clap::Arg;
use serde::{Deserialize, Serialize};

const DEFAULT_PORT: u16 = 25566;
const DEFAULT_UPSTREAM_HOST: &str = "127.0.0.1";
const DEFAULT_UPSTREAM_PORT: u16 = 25565;

/// The main server configuration struct.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
#[serde(default)]
pub struct ProxyConfig {
    #[serde(rename = "version-string")]
    pub version: String,
    pub port: u16,
    pub upstream_host: String,
    pub upstream_port: u16,
}
impl Default for ProxyConfig {
    fn default() -> Self {
        ProxyConfig {
            version: Config::get_formatted_version(crate::config::Subcommand::Proxy),
            port: DEFAULT_PORT,
            upstream_host: DEFAULT_UPSTREAM_HOST.to_owned(),
            upstream_port: DEFAULT_UPSTREAM_PORT,
        }
    }
}
impl ProxyConfig {
    pub fn instance() -> &'static Self {
        &Config::instance().proxy
    }
    pub fn load_args(&mut self) {
        self.upstream_host = ProxyArgs::instance()
            .as_ref()
            .map(|p| p.upstream_host.clone())
            .unwrap_or(DEFAULT_UPSTREAM_HOST.to_owned());
    }
}

#[derive(Debug)]
pub struct ProxyArgs {
    port: u16,
    upstream_host: String,
    upstream_port: u16,
}
impl Default for ProxyArgs {
    fn default() -> Self {
        ProxyArgs {
            port: DEFAULT_PORT,
            upstream_host: DEFAULT_UPSTREAM_HOST.to_owned(),
            upstream_port: DEFAULT_UPSTREAM_PORT,
        }
    }
}
impl ProxyArgs {
    pub fn instance() -> Option<&'static Self> {
        Args::instance().proxy.as_ref()
    }
    pub fn command() -> clap::Command {
        clap::Command::new("proxy")
            .about("Run composition in proxy mode")
            .arg(
                Arg::new("port")
                    .short('p')
                    .long("port")
                    .help("Proxy listening port")
                    .value_hint(clap::ValueHint::Other)
                    .value_parser(clap::value_parser!(u16))
                    .default_value(const_format::formatcp!("{}", DEFAULT_PORT)),
            )
            .arg(
                Arg::new("upstream-host")
                    .short('U')
                    .long("upstream-host")
                    .help("Upstream server address")
                    .value_hint(clap::ValueHint::Hostname)
                    .default_value(const_format::formatcp!("{}", DEFAULT_UPSTREAM_HOST)),
            )
            .arg(
                Arg::new("upstream-port")
                    .short('P')
                    .long("upstream-port")
                    .help("Upstream server port")
                    .value_hint(clap::ValueHint::Other)
                    .value_parser(clap::value_parser!(u16))
                    .default_value(const_format::formatcp!("{}", DEFAULT_UPSTREAM_PORT)),
            )
    }
    pub fn parse(m: clap::ArgMatches) -> Self {
        let mut proxy_args = ProxyArgs::default();
        proxy_args.port = *m.get_one::<u16>("port").unwrap_or(&proxy_args.port);
        proxy_args.upstream_host = m
            .get_one::<String>("upstream-host")
            .unwrap_or(&proxy_args.upstream_host)
            .clone();
        proxy_args.upstream_port = *m
            .get_one::<u16>("upstream-port")
            .unwrap_or(&proxy_args.upstream_port);
        proxy_args
    }
}
