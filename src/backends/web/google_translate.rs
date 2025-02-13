use anyhow::anyhow;

use crate::{Query, RespData, backends::web::google_translate_token::getTK};

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
            url_free: "https://translate.google.com/translate_a/single?".to_owned(),
            _url_voice: "https: //translate.google.cn/translate_tts?ie=UTF-8&client=t&prev
=input&q={}&tl=en&total=1&idx=0&textlen=4&tk={}"
                .to_owned(),
        }
    }
}

impl GTrans {
    // pub async fn query(&self, query: Query) -> anyhow::Result<RespData> {
    //     log::info!("requesting google translate");
    //     let client = new_client();
    //     let params = [
    //         ("client", &"gtx".into()),
    //         ("dt", &"t".into()),
    //         ("q", &query.text),
    //         ("tl", &query.lang_to),
    //         ("sl", &"auto".into()),
    //         ("ie", &"UTF-8".into()),
    //         ("oe", &"UTF-8".into()),
    //     ];

    //     let resp = client
    //         .post(&self.url_free)
    //         .form(&params)
    //         .send()
    //         .await
    //         .unwrap();
    //     log::debug!("status: {}", resp.status());
    //     if !resp.status().is_success() {
    //         return Err(anyhow!("google HTTP error with code {}", resp.status()));
    //     }
    //     let resp_data: serde_json::Value = resp.json().await?;
    //     log::debug!("raw data from google translate: {:?}", resp_data);
    //     // TODO figure out google translate's response format
    //     let t = resp_data.as_array().ok_or(anyhow!("not an array"))?[0]
    //         .as_array()
    //         .ok_or(anyhow!("not an array"))?;
    //     let mut trans_text = String::new();
    //     for item in t {
    //         let trans_sentence = item.as_array().ok_or(anyhow!("not an array"))?[0]
    //             .as_str()
    //             .ok_or(anyhow!("not a string"))?;
    //         trans_text.push_str(trans_sentence);
    //     }
    //     Ok(RespData {
    //         backend: "google translate".to_owned(),
    //         query,
    //         // short_desc: resp.text().unwrap(),
    //         basic_desc: trans_text,
    //         phonetic_symbol: None,
    //         detail_desc: None,
    //         audio: None,
    //     })
    // }

    pub async fn query(&self, query: Query) -> anyhow::Result<RespData> {
        log::info!("requesting google translate");
        let client = new_client();
        let params = [
            ("client", &"webapp".into()),
            ("sl", &"auto".into()),
            ("tl", &query.lang_to),
            ("hl", &"en".into()),
            (
                "dt",
                &"[\"at\", \"bd\", \"ex\", \"ld\", \"md\", \"qca\", \"rw\", \"rm\", \"ss\", \"t\"]"
                    .into(),
            ),
            ("source", &"bh".into()),
            ("ssel", &"0".into()),
            ("tsel", &"0".into()),
            ("kc", &"1".into()),
            ("tk", &getTK(&query.text)),
            ("q", &query.text),
            // ("ie", &"UTF-8".into()),
            // ("oe", &"UTF-8".into()),
        ];

        let resp = client
            .post(&self.url_free)
            .form(&params)
            .send()
            .await
            .unwrap();
        log::debug!("status: {}", resp.status());
        if !resp.status().is_success() {
            return Err(anyhow!("google HTTP error with code {}", resp.status()));
        }
        let resp_data: serde_json::Value = resp.json().await?;
        log::debug!("raw data from google translate: {:?}", resp_data);
        // TODO figure out google translate's response format
        let t = resp_data.as_array().ok_or(anyhow!("not an array"))?[0]
            .as_array()
            .ok_or(anyhow!("not an array"))?;
        let mut trans_text = String::new();
        for item in t {
            let trans_sentence = item.as_array().ok_or(anyhow!("not an array"))?[0]
                .as_str()
                .ok_or(anyhow!("not a string"))?;
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
