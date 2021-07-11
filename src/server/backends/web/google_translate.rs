use crate::server::{Query, RespData};
use std::sync::Arc;

use super::new_client;

fn _calc_token() {
    let tkk = "440498.1287591069";
    let _sp = tkk.split('.');
}

#[derive(Clone, Debug)]
pub struct GTrans {
    url_free: String,
    _url_voice: String,
}

impl GTrans {
    pub fn new() -> GTrans {
        GTrans {
            url_free: "https://translate.google.cn/translate_a/single?client=gtx&dt=t".to_owned(),
            _url_voice: "https: //translate.google.cn/translate_tts?ie=UTF-8&client=t&prev
=input&q={}&tl=en&total=1&idx=0&textlen=4&tk={}".to_owned(),
        }
    }
}

impl GTrans {
    pub async fn query(&self, query: Arc<Query>) -> Result<RespData, String> {
        log::info!("requesting google translate");
        let client = new_client();
        let params = [
            ("q", &query.text),
            ("tl", &query.lang_to),
            ("sl", &"auto".into()),
            ("ie", &"UTF-8".into()),
            ("oe", &"UTF-8".into()),
        ];

        let resp = client.post(&self.url_free).form(&params).send().await.unwrap();
        log::debug!("status: {}", resp.status());
        let resp_data: serde_json::Value = resp.json().await.unwrap();
        log::debug!("raw data from google translate: {:?}", resp_data);
        // TODO figure out google translate's response format
        let t = resp_data.as_array().unwrap()[0].as_array().unwrap();
        let mut trans_text = String::new();
        for item in t {
            let trans_sentence = item.as_array().unwrap()[0].as_str().unwrap();
            trans_text.push_str(trans_sentence);
        }
        Ok(RespData {
            backend: "google translate".to_owned(),
            query,
            // short_desc: resp.text().unwrap(),
            basic_desc: trans_text,
            phonetic_symbol: None,
            detail_desc: None,
            audio: None,
        })
    }
}
