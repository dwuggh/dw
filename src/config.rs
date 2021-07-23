use serde::Deserialize;
use std::fs::File;
use std::io::prelude::*;

use crate::backends::youdao::YoudaoAPIKey;
extern crate xdg;

use once_cell::sync::OnceCell;

static CONFIG: OnceCell<Config> = OnceCell::new();

pub fn init() -> Result<(), Config> {
    CONFIG.set(read_config())
}

pub fn get() -> &'static Config {
    CONFIG.get().unwrap()
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct Config {
    // pub backends: Vec<String>,
    #[serde(default)]
    pub proxy: Proxy,

    #[serde(default)]
    pub server: Option<Server>,

    #[serde(default)]
    pub youdao: Option<YoudaoAPIKey>,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct Proxy {
    // #[serde(default = "String::new")]
    pub http_proxy: Option<String>,
    // #[serde(default = "String::new")]
    pub https_proxy: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Server {
    pub addr: String,
}

pub fn read_config_from_file(
    config_path: impl AsRef<std::path::Path>,
    buf: &mut String,
) -> std::io::Result<&mut String> {
    let mut file = File::open(config_path)?;
    file.read_to_string(buf)?;
    Ok(buf)
}

pub fn read_config() -> Config {
    let xdg_dirs = xdg::BaseDirectories::with_prefix("dw").unwrap();
    let mut config_buf = String::new();
    if let Some(config_path) = xdg_dirs.find_config_file("config.toml") {
        read_config_from_file(config_path, &mut config_buf).expect("failed to read config file");
        let config: Config = toml::from_str(&config_buf).expect("cannot parse config file");
        log::info!("loaded config: {:?}", config);
        config
    } else {
        Config::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_load_empty_configs() {
        let config: Result<Config, toml::de::Error> = toml::from_str("");
        assert_eq!(true, config.is_ok())
    }

    #[test]
    fn it_can_read_proxy() {
        let config: Config = toml::from_str(
            "
[proxy]
http_proxy = \"socks5://127.0.0.1:1080\"
https_proxy = \"socks5://127.0.0.1:1080\"
",
        )
        .unwrap();
        let http_proxy = config.proxy.http_proxy.unwrap();
        assert_eq!(http_proxy, "socks5://127.0.0.1:1080")
    }
    #[test]
    fn it_can_read_youdao_api_key() {
        let config: Config = toml::from_str(
            "
[youdao]
secret_key = \"fuck\"
id = \"shit\"
",
        )
        .unwrap();
        let youdao = config.youdao.unwrap();
        assert_eq!(youdao.id, "shit");
    }

    #[test]
    fn it_can_parse_server_address() {
        let config: Config = toml::from_str(
            "
[server]
addr = \"127.0.0.1:1080\"
",
        )
        .unwrap();
        let addr = config.server.unwrap().addr;
        assert_eq!(addr, "127.0.0.1:1080");
    }
}
