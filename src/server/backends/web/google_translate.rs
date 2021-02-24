use crate::server::config::{ConfigRef, Proxy};

use crate::server::{Backend, Query, RespData};
use std::sync::Arc;

use super::new_client_blocking;

fn _calc_token() {
    let tkk = "440498.1287591069";
    let _sp = tkk.split('.');
}

pub struct GTrans {
    url_free: String,
    url_voice: String,
    proxy: Proxy,
}

impl GTrans {
    pub fn new(config: ConfigRef) -> GTrans {
        GTrans {
            url_free: "https://translate.google.cn/translate_a/single?client=gtx&dt=t".to_owned(),
            url_voice: "https: //translate.google.cn/translate_tts?ie=UTF-8&client=t&prev=input&q={}&tl=en&total=1&idx=0&textlen=4&tk={}".to_owned(),
            proxy: config.proxy.clone()
        }
    }
}

unsafe impl Send for GTrans {}
unsafe impl Sync for GTrans {}

impl Backend for GTrans {
    fn query(&self, query: Arc<Query>) -> Result<RespData, String> {
        let client = new_client_blocking(&self.proxy);
        let params = [
            ("q", &query.text),
            ("tl", &query.lang_to),
            ("sl", &"auto".into()),
            ("ie", &"UTF-8".into()),
            ("oe", &"UTF-8".into()),
        ];

        let resp = client.post(&self.url_free).form(&params).send().unwrap();
        let resp_data: serde_json::Value = resp.json().unwrap();
        log::debug!("{:?}", resp_data);
        let t = resp_data.as_array().unwrap()[0].as_array().unwrap()[0]
            .as_array()
            .unwrap()[0]
            .as_str()
            .unwrap();
        Ok(RespData {
            backend: "google translate".to_owned(),
            query,
            // short_desc: resp.text().unwrap(),
            basic_desc: t.to_owned(),
            phonetic_symbol: None,
            detail_desc: None,
            audio: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_translate_words() {
        // let g = GTrans::new();
        // let query = Query::new("fuck", "auto", "zh", false);
        // let res = g.query(Arc::new(query)).unwrap().short_desc;
        // assert_eq!(res, "他妈的")
    }
}
