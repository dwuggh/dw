use crate::cli::config::Proxy;

use super::super::types::*;
use reqwest;
use std::sync::Arc;

fn calc_token() {
    let tkk = "440498.1287591069";
    let sp = tkk.split('.');
}

pub struct GTrans {
    url_free: String,
    url_voice: String,
    proxy: Proxy,
}

impl GTrans {
    pub fn new(proxy: Proxy) -> GTrans {
        GTrans {
            url_free: "https://translate.google.cn/translate_a/single?client=gtx&dt=t&ie=UTF-8&oe=UTF-8&sl=auto".to_owned(),
            url_voice: "https: //translate.google.cn/translate_tts?ie=UTF-8&client=t&prev=input&q={}&tl=en&total=1&idx=0&textlen=4&tk={}".to_owned(),
            proxy
        }
    }
}

impl Backend for GTrans {
    fn query<'a>(&self, query: Arc<Query<'a>>) -> Result<WordData<'a>, String> {
        let mut client_builder = reqwest::blocking::Client::builder();
        if let Some(http_proxy) = &self.proxy.http_proxy {
            client_builder = client_builder.proxy(reqwest::Proxy::http(http_proxy).unwrap());
        }
        if let Some(https_proxy) = &self.proxy.https_proxy {
            client_builder = client_builder.proxy(reqwest::Proxy::https(https_proxy).unwrap());
        }
        let client = client_builder.build().unwrap();
        let params = [("q", query.text), ("tl", &query.lang_to)];

        let resp = client.post(&self.url_free).form(&params).send().unwrap();
        let resp_data: serde_json::Value = resp.json().unwrap();
        log::debug!("{:?}", resp_data);
        let t = resp_data.as_array().unwrap()[0].as_array().unwrap()[0]
            .as_array()
            .unwrap()[0]
            .as_str()
            .unwrap();
        Ok(WordData {
            backend: "google translate".to_owned(),
            query,
            // short_desc: resp.text().unwrap(),
            short_desc: t.to_owned(),
            phonetic_symbol: None,
            long_desc: None,
            audio: None,
        })
    }
}
