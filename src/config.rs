use std::fs;
use std::path::Path;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Exclusions {
    pub dir: Vec<String>,
    pub ext: Vec<String>,
}

#[derive(Deserialize)]
pub struct Config {
    pub excluded: Exclusions,
}

pub fn get_config() -> Config {
    let path = Path::new("config.toml");
    let config_result = fs::read_to_string(path);
    let config: Config = toml::from_str(config_result.unwrap().as_str()).unwrap();
    return config;
}