use crate::history::History;
use crate::runner;
use crate::{formatter, Query};
use std::sync::Mutex;

use futures::{FutureExt, TryFutureExt, StreamExt, SinkExt};
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use warp::ws::WebSocket;
use warp::Filter;

static RUNNER: OnceCell<runner::Runner> = OnceCell::new();
static HISTORY: OnceCell<Mutex<History>> = OnceCell::new();

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Params {
    query: Query,
    format: formatter::Format,
}

impl Params {
    pub fn new(query: Query, format: formatter::Format) -> Self {
        Self { query, format }
    }
}

async fn lookup_handler(params: Params) -> Result<impl warp::Reply, std::convert::Infallible> {
    let query = params.query;
    if query.is_short_text {
        let mut h = HISTORY.get().unwrap().lock().unwrap();
        h.add(&query.text, &query.lang_from);
        h.dump().unwrap();
    }
    log::info!("incoming query: {:?}", query);
    let format = params.format;
    let mut rx = RUNNER.get().unwrap().run(query, format).await;
    let mut resp = String::new();
    while let Some(Some(text)) = rx.recv().await {
        resp.push_str("\n\n");
        resp.push_str(&text);
    }
    Ok(warp::reply::json(&resp))
}

async fn ws_handler(websocket: WebSocket) -> anyhow::Result<()> {
    let (mut tx, mut rx) = websocket.split();
    while let Some(Ok(ref msg)) = rx.next().await {
        log::debug!("{:?}", msg);
        if msg.is_text() {
            let msg = msg.to_str().unwrap();
            let params: Params = serde_json::from_str(msg)?;
            let query = params.query;
            if query.is_short_text {
                let mut h = HISTORY.get().unwrap().lock().unwrap();
                h.add(&query.text, &query.lang_from);
                h.dump().unwrap();
            }
            log::info!("incoming query: {:?}", query);
            let format = params.format;
            let mut rx = RUNNER.get().unwrap().run(query, format).await;
            while let Some(text) = rx.recv().await {
                if let Some(text) = text {
                    tx.send(warp::ws::Message::text(text)).await?;
                }
            }
            tx.send(warp::ws::Message::binary([0b0, ])).await?
        }
    }
    tx.send(warp::ws::Message::close()).await?;
    Ok(())
}

pub async fn init_server(addr: &str) -> tokio::io::Result<()> {
    match RUNNER.set(runner::Runner::new()) {
        Ok(it) => it,
        _ => unreachable!(),
    };
    match HISTORY.set(Mutex::new(History::new())) {
        Ok(it) => it,
        _ => unreachable!(),
    };

    // POST /lookup
    let lookup = warp::post()
        .and(warp::path("lookup"))
        .and(warp::body::json())
        .and_then(lookup_handler);
    let ws = warp::path("ws").and(warp::ws()).map(|ws: warp::ws::Ws| {
        ws.on_upgrade(|websocket| async {
            ws_handler(websocket).await;
        })
    });

    let hello = warp::path("hello").map(|| warp::reply::json(&"dw".to_string()));
    let last_word = warp::path("last_word")
        .map(|| warp::reply::json(&HISTORY.get().unwrap().lock().unwrap().last_word().unwrap()));

    let routes = ws.or(lookup).or(hello).or(last_word);
    warp::serve(routes)
        .run(addr.parse::<std::net::SocketAddr>().unwrap())
        .await;

    Ok(())
}
