use super::new_client;
use crate::{Query, RespData};
use anyhow::anyhow;
use scraper::{Html, Selector};

#[derive(Clone, Debug)]
pub struct DictCN {
    url: String,
}

impl DictCN {
    pub fn new() -> DictCN {
        DictCN {
            url: "https://dict.cn/search".to_string(),
        }
    }
}

impl DictCN {
    pub async fn query(&self, query: Query) -> anyhow::Result<RespData> {
        log::info!("requesting dict.cn");
        let client = new_client();
        let resp = client
            .post(&self.url)
            .form(&[("q", query.text)])
            .send()
            .await
            .unwrap();
        log::info!("{:?}", resp);
        let resp_text = resp.text().await.unwrap();
        log::info!("{}", resp_text);
        let resp_html = Html::parse_document(&resp_text);
        let selector_detail = Selector::parse("layout detail").unwrap();
        let res = resp_html.select(&selector_detail).next().unwrap();
        log::info!("{:?}", res);

        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    fn test_dictcn() {
        crate::config::init().unwrap();
        env_logger::builder()
            .is_test(true)
            // .filter_level(log::LevelFilter::Debug)
            .try_init()
            .unwrap();
        let d = DictCN::new();
        let query = Query::new("fuck", "en", "zh", false);
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        let a = rt.block_on(d.query(query)).unwrap();
    }
}
