use std::sync::Arc;

use crate::server::backends::google_translate::GTrans;
use crate::server::backends::youdao::Youdao;
use crate::server::{Backend, Query, WordData};
use crate::cli::config::Config;

pub struct Runner {
    backends: Vec<Box<dyn Backend>>,
}

impl Runner {
    pub fn new(config: Config) -> Self {
        let mut backends: Vec<Box<dyn Backend>> = Vec::new();
        backends.push(Box::new(GTrans::new(config.proxy.clone())));
        backends.push(Box::new(Youdao::new(config.clone())));

        Runner { backends }
    }

    pub fn run<'a>(&self, query: Arc<Query>) -> Vec<WordData> {
        let mut result: Vec<WordData> = Vec::new();
        // TODO concurrent code
        for backend in &self.backends {
            match backend.query(Arc::clone(&query)) {
                Ok(res) => {
                    result.push(res);
                }
                Err(e) => {
                    println!("error: {}", e);
                }
            }
        }
        result
    }
}
