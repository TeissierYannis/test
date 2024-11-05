use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
pub struct Config {
    pub activate: bool,
    pub addr: String,
    pub port: u16,
    pub log_level: String,
    pub xdp: bool,
    pub netfilter: bool,
    pub localnets: Vec<String>,
    pub htable_size: usize,
}

impl Config {
    pub fn load() -> Config {
        let content = fs::read_to_string("config.toml").expect("Failed to read config.toml");
        toml::from_str(&content).expect("Invalid configuration format")
    }
}
