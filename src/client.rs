
use futures::{StreamExt, SinkExt};
use tokio_tungstenite::{connect_async, tungstenite::Message};
use anyhow::Result;

use crate::server::Params;


pub async fn lookup_ws_single(uri: &str, params: &Params) -> Result<()> {
    log::info!("url: {}", uri);
    let (ws_stream, _) = connect_async(uri).await?;
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
            _ => ()
        }
    }
    Ok(())
}
