use std::sync::Arc;
use tokio::sync::mpsc;

use super::backends::google_translate::GTrans;
use super::backends::youdao::Youdao;
use super::formatter::Formatter;
use super::{Backend, Query};

pub struct Runner {
    backends: Vec<Arc<Backend>>,
}

impl Runner {
    pub fn new() -> Self {
        let mut backends = Vec::new();
        backends.push(Arc::new(Backend::GTrans(GTrans::new())));
        backends.push(Arc::new(Backend::Youdao(Youdao::new())));

        Runner { backends }
    }

    pub async fn run(&self, query: Arc<Query>, formatter: Formatter) -> mpsc::Receiver<String> {
        // let mut results = Vec::new();
        let (tx, rx) = mpsc::channel(32);
        for backend in &self.backends {
            let backend = Arc::clone(backend);
            let q = Arc::clone(&query);
            let tx = tx.clone();
            log::debug!("running backend {:?}", backend);
            let handle = tokio::spawn(async move {
                let resp = match backend.query(q).await {
                    Ok(res) => formatter.format(&res),
                    Err(e) => {
                        log::error!("query error: {}", e);
                        eprintln!("{}", e);
                        // TODO
                        "error".to_string()
                    }
                };
                if let Err(e) = tx.send(resp).await {
                    log::error!("channel send error: {}", e);
                }
            })
            .await;
            match handle {
                Ok(()) => (),
                Err(e) => log::error!("tokio task error: {}", e),
            }
        }
        // results.join("\n\n").to_string()
        rx
    }
}
