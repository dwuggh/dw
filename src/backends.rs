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
    pub async fn query(&self, query: Query) -> anyhow::Result<RespData> {
        match self {
            Backend::Youdao(youdao) => youdao.query(query).await,
            Backend::GTrans(gtrans) => gtrans.query(query).await,
            Backend::MDict(mdict) => mdict.query(query).await,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Query;

    use super::google_translate::GTrans;

    fn query_en(text: &str) -> Query {
        let query = Query::new(text, "en", "zh", false);
        query
    }

    fn query_fuck() -> Query {
        query_en("fuck")
    }

    // #[test]
    fn google_translate_can_translate_fuck() {
        crate::config::init().unwrap();
        let _ = env_logger::builder().is_test(true).try_init();
        let g = GTrans::new();
        let query = query_fuck();
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        let resp = rt.block_on(g.query(query)).unwrap();
        assert_eq!(resp.basic_desc, "他妈的")
    }
}
