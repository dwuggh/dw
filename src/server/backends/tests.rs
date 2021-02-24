use std::sync::Arc;

use crate::server::{Backend, Query, RespData};

use super::super::config::{Config, ConfigRef};
use super::google_translate::GTrans;
use super::youdao::Youdao;

fn new_empty_config() -> ConfigRef {
    let config = Config::default();
    ConfigRef::from(config)
}

fn query_en(text: &str) -> Arc<Query> {
    let query = Query::new(text, "en", "zh", false);
    Arc::new(query)
}

fn query_fuck() -> Arc<Query> {
    query_en("fuck")
}

#[test]
fn google_translate_can_translate_fuck() {
    let config = new_empty_config();
    let g = GTrans::new(config);
    let query = query_fuck();
    let resp = g.query(Arc::clone(&query)).unwrap();
    assert_eq!(resp.basic_desc, "他妈的")
}
