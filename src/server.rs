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
    let res = RUNNER.get().unwrap().run(query, format).await;
    Ok(warp::reply::json(&res))
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

    warp::serve(lookup)
        .run(addr.parse::<std::net::SocketAddr>().unwrap())
        .await;

    Ok(())
}
