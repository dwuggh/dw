use std::{rc::Rc, sync::Arc};

use super::backends::google_translate::GTrans;
use super::backends::youdao::Youdao;
use super::{Backend, Query, WordData};
use super::config::ConfigRef;

pub struct Runner {
    backends: Vec<Box<dyn Backend>>,
}

impl Runner {
    pub fn new(config: ConfigRef) -> Self {
        let mut backends: Vec<Box<dyn Backend>> = Vec::new();
        backends.push(Box::new(GTrans::new(Rc::clone(&config))));
        backends.push(Box::new(Youdao::new(Rc::clone(&config))));

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
