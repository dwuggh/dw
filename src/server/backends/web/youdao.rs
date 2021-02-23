use crate::server::config::{Config, Proxy};

use crate::server::{Backend, Query, WordData};
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct YoudaoAPIKey {
    pub secret_key: String,
    pub id: String,
}

pub struct Youdao {
    url_free: String,
    api_key: Option<YoudaoAPIKey>,
    proxy: Proxy,
}

impl Youdao {
    pub fn new(config: Config) -> Youdao {
        Youdao {
            url_free: "https://openapi.youdao.com/api".to_owned(),
            api_key: config.youdao,
            proxy: config.proxy,
        }
    }
}

impl Backend for Youdao {
    fn query(&self, query: Arc<Query>) -> Result<WordData, String> {
        // TODO
        match &self.api_key {
            Some(api_key) => {

                let mut client_builder = reqwest::blocking::Client::builder();
                if let Some(http_proxy) = &self.proxy.http_proxy {
                    client_builder = client_builder.proxy(reqwest::Proxy::http(http_proxy).unwrap());
                }
                if let Some(https_proxy) = &self.proxy.https_proxy {
                    client_builder = client_builder.proxy(reqwest::Proxy::https(https_proxy).unwrap());
                }
                let client = client_builder.build().unwrap();
                // https://ai.youdao.com/DOCSIRMA/html/%E8%87%AA%E7%84%B6%E8%AF%AD%E8%A8%80%E7%BF%BB%E8%AF%91/API%E6%96%87%E6%A1%A3/%E6%96%87%E6%9C%AC%E7%BF%BB%E8%AF%91%E6%9C%8D%E5%8A%A1/%E6%96%87%E6%9C%AC%E7%BF%BB%E8%AF%91%E6%9C%8D%E5%8A%A1-API%E6%96%87%E6%A1%A3.html
                let salt = Uuid::new_v4().to_string();
                let text = &query.text;
                let curtime = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs()
                    .to_string();
                let mut sign = String::new();
                sign.push_str(&api_key.id);
                if text.len() <= 20 {
                    sign.push_str(text);
                } else {
                    let l = text.len();
                    let beg = &text[..10];
                    let end = &text[(l - 10)..l];
                    sign.push_str(beg);
                    sign.push_str(&l.to_string());
                    sign.push_str(end);
                }
                sign.push_str(&salt);
                sign.push_str(&curtime);
                sign.push_str(&api_key.secret_key);
                let mut hasher = Sha256::new();
                hasher.update(sign);
                sign = format!("{:X}", hasher.finalize());

                let params = [
                    ("q", &query.text),
                    ("from", &query.lang_from),
                    ("to", &query.lang_to),
                    ("appKey", &api_key.id),
                    ("salt", &salt),
                    ("sign", &sign),
                    ("signType", &"v3".into()),
                    ("curtime", &curtime),
                    // ("ext", &"TODO".into()),
                    ("strict", &"false".into()),
                ];

                log::debug!("{:?}", params);
                let resp = client.post(&self.url_free).form(&params).send().unwrap();
                let resp_data: Value = resp.json().unwrap();
                log::debug!("{:?}", resp_data);
                let error_code = resp_data.get("errorCode").unwrap().as_str().unwrap();
                if error_code != "0" {
                    return Err(error_code.to_string());
                }
                let trans_list: Vec<&str> = resp_data.get("translation").unwrap().as_array().unwrap().into_iter().map(move |v: &Value| {v.as_str().unwrap()}).collect();
                let trans = trans_list.join("\n");
                // let basic = resp_data.get("basic");
                Ok(WordData {
                    backend: "youdao translate".to_owned(),
                    query,
                    // short_desc: resp.text().unwrap(),
                    short_desc: trans,
                    phonetic_symbol: None,
                    long_desc: None,
                    audio: None,
                })
            }
            None => {
                Err(String::from("no youdao API key"))
            }
        }

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_translate_words() {}
}
