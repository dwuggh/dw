#![allow(dead_code)]
use anyhow::Result;
use futures::{SinkExt, StreamExt};
use tokio_tungstenite::{connect_async, tungstenite::Message};

use crate::server::Params;

pub async fn lookup_ws_single(addr: &str, params: &Params) -> Result<()> {
    let url = &format!("ws://{}/ws", addr);
    log::info!("websocket url: {}", url);
    let (ws_stream, _) = connect_async(url).await?;
    log::info!("WebSocket handshake has been successfully completed");
    let (mut write, mut read) = ws_stream.split();
    write.send(serde_json::to_string(params)?.into()).await?;
    while let Some(msg) = read.next().await {
        let msg = msg?;
        match msg {
            Message::Text(text) => {
                println!("{}", text);
            }
            Message::Binary(v) => {
                if v.len() == 1 && v[0] == 0 {
                    write.close().await?;
                }
            }
            _ => (),
        }
    }
    Ok(())
}

pub async fn lookup_http_post(addr: &str, params: &Params) {
    let client = reqwest::Client::new();
    let res = client
        .post(format!("http://{}/lookup", addr))
        .json(&params)
        .send()
        .await
        .unwrap();
    let res = res.json::<String>().await.unwrap();
    println!("{}", res);
}
