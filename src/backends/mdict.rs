use std::sync::Arc;

use mdict::mdict::MDict;

use crate::{Query, RespData};

#[derive(Clone, Debug)]
pub struct MDictBackend {
    dict: MDict,
}

impl MDictBackend {
    pub fn new(mdx_path: &str, mdd_path: &str) -> MDictBackend {
        let dict = MDict::new(mdx_path, Some(mdd_path.to_string())).unwrap();
        MDictBackend { dict }
    }

    pub async fn query(&self, query: Arc<Query>) -> Result<RespData, String> {
        let text = self
            .dict
            .lookup(&query.text)
            .ok_or("cannot lookup".to_string())?;
        Ok(RespData {
            backend: "mdict".to_string(),
            query,
            basic_desc: text.clone(),
            phonetic_symbol: None,
            detail_desc: None,
            audio: None,
        })
    }
}
