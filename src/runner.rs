use std::sync::Arc;
use tokio::sync::mpsc;

use crate::Query;

use super::backends::google_translate::GTrans;
use super::backends::youdao::Youdao;
use super::backends::Backend;
use super::formatter::Format;

pub struct Runner {
    backends: Vec<Arc<Backend>>,
}

impl Runner {
    pub fn new() -> Self {
        let mut backends = Vec::new();
        backends.push(Arc::new(Backend::GTrans(GTrans::new())));
        backends.push(Arc::new(Backend::Youdao(Youdao::new())));
        // let mdd_path = "/home/dwuggh/.dicts/OALDcn8/oald.mdd";
        // let mdx_path = "/home/dwuggh/.dicts/OALDcn8/oald.mdx";
        // backends.push(Arc::new(Backend::MDict(MDictBackend::new(
        //     mdx_path, mdd_path,
        // ))));

        Runner { backends }
    }

    pub async fn run(&self, query: Query, format: Format) -> mpsc::Receiver<Option<String>> {
        let (tx, rx) = mpsc::channel(32);
        let _handles: Vec<tokio::task::JoinHandle<()>> = self
            .backends
            .iter()
            .map(|backend| {
                let backend = Arc::clone(backend);
                let q = query.clone();
                let tx = tx.clone();
                log::debug!("running backend {:?}", backend);
                let handle = tokio::task::spawn(async move {
                    let resp = match backend.query(q).await {
                        Ok(res) => Some(format.format(&res)),
                        Err(e) => {
                            log::error!("query error: {}", e);
                            None
                        }
                    };
                    if let Err(e) = tx.send(resp).await {
                        log::error!("channel send error: {}", e);
                    }
                });
                handle
            })
            .collect();
        rx
    }
}
