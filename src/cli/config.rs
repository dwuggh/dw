use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;

use crate::server::backends::youdao::YoudaoAPIKey;
extern crate xdg;

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    // pub backends: Vec<String>,
    #[serde(default)]
    pub proxy: Proxy,

    #[serde(default)]
    pub youdao: Option<YoudaoAPIKey>
}


#[derive(Deserialize, Debug, Default, Clone)]
pub struct Proxy {
    // #[serde(default = "String::new")]
    pub http_proxy: Option<String>,
    // #[serde(default = "String::new")]
    pub https_proxy: Option<String>,
}

#[derive(Deserialize, Debug, Default)]
pub struct Backend {
    pub youdao_api_key: Option<YoudaoAPIKey>
}

pub fn read_config() -> Config {
    let xdg_dirs = xdg::BaseDirectories::with_prefix("dw").unwrap();
    let mut config_buf = String::new();
    if let Some(config_path) = xdg_dirs.find_config_file("config.toml") {
        let mut file = File::open(config_path).unwrap();
        file.read_to_string(&mut config_buf).unwrap();
    }
    let config: Config = toml::from_str(&config_buf).unwrap();
    log::info!("loaded config: {:?}", config);
    config
}
