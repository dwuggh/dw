use std::sync::Arc;
use tokio::sync::mpsc;

use super::backends::google_translate::GTrans;
use super::backends::youdao::Youdao;
use super::backends::Backend;
use super::formatter::Formatter;
use super::Query;

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
        // let mut results: Vec<String> = Vec::new();
        let (tx, rx) = mpsc::channel(32);
        let handles: Vec<_> = self
            .backends
            .iter()
            .map(|backend| -> tokio::task::JoinHandle<()> {
                let backend = Arc::clone(backend);
                let q = Arc::clone(&query);
                let tx = tx.clone();
                log::debug!("running backend {:?}", backend);
                let handle = tokio::task::spawn(async move {
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
                });
                handle
            })
            .collect();
        futures::future::join_all(handles)
            .await
            .iter()
            .for_each(|res| {
                if let Err(e) = res {
                    log::error!("tokio task error: {}", e)
                }
            });
        // results.join("\n\n").to_string()
        rx
    }
}
