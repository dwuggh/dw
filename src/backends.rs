mod mdict;
mod web;

pub use self::mdict::MDictBackend;
pub use web::*;

use super::{Query, RespData};

/// Backend for searching words. Can be dictserver, mdd/mdx, or online searching.
#[derive(Clone, Debug)]
pub enum Backend {
    Youdao(youdao::Youdao),
    GTrans(google_translate::GTrans),
    MDict(mdict::MDictBackend),
}

impl Backend {
    pub async fn query(&self, query: std::sync::Arc<Query>) -> Result<RespData, String> {
        match self {
            Backend::Youdao(youdao) => youdao.query(query).await,
            Backend::GTrans(gtrans) => gtrans.query(query).await,
            Backend::MDict(mdict) => mdict.query(query).await,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::Query;

    use super::google_translate::GTrans;

    fn query_en(text: &str) -> Arc<Query> {
        let query = Query::new(text, "en", "zh", false);
        Arc::new(query)
    }

    fn query_fuck() -> Arc<Query> {
        query_en("fuck")
    }

    #[test]
    fn google_translate_can_translate_fuck() {
        crate::config::init().unwrap();
        let g = GTrans::new();
        let query = query_fuck();
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        let resp = rt.block_on(g.query(Arc::clone(&query))).unwrap();
        assert_eq!(resp.basic_desc, "他妈的")
    }
}
