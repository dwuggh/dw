use crate::server::config::Proxy;
use reqwest;

pub mod google_translate;
mod google_translate_token;
pub mod youdao;

pub fn new_client_blocking(proxy: &Proxy) -> reqwest::blocking::Client {
    let mut client_builder = reqwest::blocking::Client::builder();
    if let Some(http_proxy) = &proxy.http_proxy {
        client_builder = client_builder.proxy(reqwest::Proxy::http(http_proxy).unwrap());
    }
    if let Some(https_proxy) = &proxy.https_proxy {
        client_builder = client_builder.proxy(reqwest::Proxy::https(https_proxy).unwrap());
    }
    client_builder.build().unwrap()
}
