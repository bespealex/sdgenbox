use std::{net::Ipv4Addr, path::Path};

/// Configuration variables of sdgenbox
#[derive(Debug, Clone, serde::Deserialize)]
pub struct Config {
    pub host: Ipv4Addr,
    pub port: u16,

    pub database_url: String,
    pub media_root: Box<Path>,
}
