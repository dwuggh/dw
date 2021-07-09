
use std::sync::Arc;

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

    pub async fn run(&self, query: Arc<Query>, formatter: Formatter) {
        for backend in &self.backends {
            let backend = Arc::clone(backend);
            let q = Arc::clone(&query);
            log::debug!("running backend {:?}", backend);
            tokio::task::spawn(async move {
                match backend.query(q).await {
                    Ok(res) => {
                        println!("{}", formatter.format(&res))
                    }
                    Err(e) => {
                        log::error!("query error: {}", e);
                        eprintln!("{}", e)
                    }
                }
            }).await.unwrap();
        }
    }
}
