pub mod backends;
pub mod config;
pub mod formatter;
mod history;
pub mod runner;
pub mod transformer;
pub mod types;

use std::sync::{Arc, Mutex};

pub use history::History;
pub use types::*;

use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use warp::Filter;

static RUNNER: OnceCell<runner::Runner> = OnceCell::new();
static HISTORY: OnceCell<Mutex<History>> = OnceCell::new();

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Params {
    query: Query,
    format: formatter::Formatter,
}

impl Params {
    pub fn new(query: Query, format: formatter::Formatter) -> Self {
        Self { query, format }
    }
}

async fn lookup_handler(params: Params) -> Result<impl warp::Reply, std::convert::Infallible> {
    let query = Arc::new(params.query);
    if query.is_short_text {
        let mut h = HISTORY.get().unwrap().lock().unwrap();
        h.add(&query.text, &query.lang_from);
        h.dump().unwrap();
    }
    log::info!("incoming query: {:?}", query);
    let format = params.format;
    let mut rx = RUNNER.get().unwrap().run(query, format).await;
    let mut resp = String::new();
    while let Some(text) = rx.recv().await {
        resp.push_str("\n\n");
        resp.push_str(&text);
    }
    Ok(warp::reply::json(&resp))
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

    let hello = warp::path("hello").map(|| warp::reply::json(&"dw".to_string()));

    warp::serve(lookup.or(hello))
        .run(addr.parse::<std::net::SocketAddr>().unwrap())
        .await;

    Ok(())
}
