use crate::{config, Query, RespData};
use async_trait::async_trait;
use reqwest::{self, Response};

pub mod dictcn;
pub mod google_translate;
mod google_translate_token;
pub mod youdao;

pub fn new_client() -> reqwest::Client {
    let mut client_builder = reqwest::Client::builder();
    let proxy = &config::get().proxy;
    if let Some(http_proxy) = &proxy.http_proxy {
        client_builder = client_builder.proxy(reqwest::Proxy::http(http_proxy).unwrap());
    }
    if let Some(https_proxy) = &proxy.https_proxy {
        client_builder = client_builder.proxy(reqwest::Proxy::https(https_proxy).unwrap());
    }
    client_builder.build().unwrap()
}

#[async_trait]
pub trait WebClient {
    fn req_params(&self, query: &Query) -> anyhow::Result<Vec<(String, String)>>;
    fn parse_resp(&self, resp: Response) -> anyhow::Result<RespData>;
    fn get_url(&self) -> &str;
    async fn send_req(&self, query: &Query) -> anyhow::Result<Response> {
        let client = new_client();
        let params = self.req_params(query)?;
        let resp = client.post(self.get_url()).form(&params).send().await?;
        Ok(resp)
    }
    async fn query(&self, query:&Query) -> anyhow::Result<RespData> {
        self.parse_resp(self.send_req(query).await?)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum JSONValueType {
    String(String),
    Index(usize)
}

fn construct_selector_json(input: Vec<JSONValueType>) -> Box<dyn Fn(&str) -> anyhow::Result<RespData>> {
    let re = move |resp: &str| -> anyhow::Result<RespData> {
        let json = serde_json::to_value(resp)?;
        let mut json_ref = &json;
        for it in &input {
            || -> Option<()> {
                match it {
                    JSONValueType::String(s) => {
                        json_ref = json_ref.as_object()?.get(s)?;
                    }
                    JSONValueType::Index(i) => {
                        json_ref = json_ref.as_array()?.get(*i)?;
                    }
                }
                return Some(())
            }().ok_or(anyhow::anyhow!("json decode error"))?;
        }
        todo!()
    };
    return Box::new(re);
}
