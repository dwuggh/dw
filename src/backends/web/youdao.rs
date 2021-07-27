use crate::config;
use crate::{Query, RespData};
use serde::Deserialize;
use serde_json::Value;
use sha2::{Digest, Sha256};
use uuid::Uuid;

use super::new_client;

#[derive(Deserialize, Debug, Clone)]
pub struct YoudaoAPIKey {
    pub secret_key: String,
    pub id: String,
}

#[derive(Clone, Debug)]
pub struct Youdao {
    url_free: String,
    api_key: Option<YoudaoAPIKey>,
}

impl Youdao {
    pub fn new() -> Youdao {
        Youdao {
            url_free: "https://openapi.youdao.com/api".to_owned(),
            api_key: config::get().youdao.clone(),
        }
    }
}

unsafe impl Send for Youdao {}
unsafe impl Sync for Youdao {}

impl Youdao {
    pub async fn query(&self, query: Query) -> Result<RespData, String> {
        log::info!("requesting youdao translate");
        match &self.api_key {
            Some(api_key) => {
                let client = new_client();
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
                // utf-8 length
                let text_len = text.chars().count();
                if text_len <= 20 {
                    sign.push_str(text);
                } else {
                    let beg: String = text.chars().take(10).collect();
                    let end: String = text
                        .chars()
                        .rev()
                        .take(10)
                        .collect::<String>()
                        .chars()
                        .rev()
                        .collect();
                    sign.push_str(&beg);
                    sign.push_str(&text_len.to_string());
                    sign.push_str(&end);
                }
                sign.push_str(&salt);
                sign.push_str(&curtime);
                sign.push_str(&api_key.secret_key);
                let mut hasher = Sha256::new();
                hasher.update(sign);
                sign = format!("{:X}", hasher.finalize());
                let lang_from = Youdao::map_lang(&query.lang_from);
                let lang_to = Youdao::map_lang(&query.lang_to);

                let params = [
                    ("q", &query.text),
                    ("from", &lang_from),
                    ("to", &lang_to),
                    ("appKey", &api_key.id),
                    ("salt", &salt),
                    ("sign", &sign),
                    ("signType", &"v3".into()),
                    ("curtime", &curtime),
                    // audio
                    // ("ext", &"TODO".into()),
                    ("strict", &"false".into()),
                ];

                let resp = client
                    .post(&self.url_free)
                    .form(&params)
                    .send()
                    .await
                    .unwrap();
                let resp_data: Value = resp.json().await.unwrap();
                log::debug!("raw data from youdao translate: {:?}", resp_data);
                let error_code = resp_data.get("errorCode").unwrap().as_str().unwrap();
                if error_code != "0" {
                    return Err(error_code.to_string());
                }
                // get lang_from and lang_to
                let langs: Vec<&str> = resp_data
                    .get("l")
                    .unwrap()
                    .as_str()
                    .unwrap()
                    .split('2')
                    .collect();
                let lang_from = langs[0];
                let _lang_to = langs[1];

                // only exists when looking up a word
                if let Some(basic) = resp_data.get("basic") {
                    if lang_from == "en" {
                        log::debug!("{:?}", basic);

                        // phonetics
                        let mut ps: Vec<(String, String)> = Vec::new();
                        if let Some(us_phonetic) = basic.get("us-phonetic") {
                            let us_phonetic = us_phonetic.as_str().unwrap().to_string();
                            ps.push(("us".to_string(), format!("/{}/", us_phonetic)));
                        }
                        if let Some(uk_phonetic) = basic.get("uk-phonetic") {
                            let uk_phonetic = uk_phonetic.as_str().unwrap().to_string();
                            ps.push(("us".to_string(), format!("/{}/", uk_phonetic)));
                        }

                        // explains
                        let explains: String = parse_explains_field(basic);
                        return Ok(RespData {
                            backend: "youdao translate".to_owned(),
                            query,
                            // short_desc: resp.text().unwrap(),
                            basic_desc: explains,
                            phonetic_symbol: Some(ps),
                            detail_desc: None,
                            audio: None,
                        });
                    } else {
                        // if not english, only word explanation
                        let trans = parse_explains_field(basic);
                        return Ok(RespData {
                            backend: "youdao translate".to_owned(),
                            query,
                            // short_desc: resp.text().unwrap(),
                            basic_desc: trans,
                            phonetic_symbol: None,
                            detail_desc: None,
                            audio: None,
                        });
                    }
                }

                // translation, always exist
                let trans_list: Vec<&str> = resp_data
                    .get("translation")
                    .unwrap()
                    .as_array()
                    .unwrap()
                    .into_iter()
                    .map(move |v: &Value| v.as_str().unwrap())
                    .collect();
                let trans = trans_list.join("\n");
                // let basic = resp_data.get("basic");
                Ok(RespData {
                    backend: "youdao translate".to_owned(),
                    query,
                    // short_desc: resp.text().unwrap(),
                    basic_desc: trans,
                    phonetic_symbol: None,
                    detail_desc: None,
                    audio: None,
                })
            }
            None => Err(String::from("no youdao API key")),
        }
    }
}

impl Youdao {
    fn map_lang(lang_code: &str) -> String {
        if lang_code == "zh" {
            String::from("zh-CHS")
        } else {
            lang_code.to_string()
        }
    }
}

fn parse_explains_field(basic: &Value) -> String {
    basic
        .get("explains")
        .unwrap()
        .as_array()
        .unwrap()
        .into_iter()
        .fold(String::new(), |mut x: String, y: &Value| {
            x.push_str(y.as_str().unwrap());
            x.push_str(";\t");
            x
        })
}
