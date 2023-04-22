use std::net::Ipv4Addr;

/// Configuration variables of sdgenbox
#[derive(Debug, serde::Deserialize)]
pub struct Config {
    pub host: Ipv4Addr,
    pub port: u16,

    pub database_url: String,
}
