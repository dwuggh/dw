use std::rc::Rc;
use std::sync::Arc;

use super::backends::google_translate::GTrans;
use super::backends::youdao::Youdao;
use super::config::ConfigRef;
use super::{Backend, Query, RespData};
use crossbeam::thread;

pub struct Runner {
    backends: Vec<Box<dyn Backend>>,
}

pub trait Handler: Send + Sync {
    type Result;
    fn handle(&self, resp: RespData) -> Self::Result;
}

impl Runner {
    pub fn new(config: ConfigRef) -> Self {
        let mut backends: Vec<Box<dyn Backend>> = Vec::new();
        backends.push(Box::new(GTrans::new(Rc::clone(&config))));
        backends.push(Box::new(Youdao::new(Rc::clone(&config))));

        Runner { backends }
    }

    pub fn run<H: Handler>(&self, query: Arc<Query>, handler: Arc<H>) {
        thread::scope(|s| {
            for backend in &self.backends {
                let q = Arc::clone(&query);
                let h = Arc::clone(&handler);
                s.spawn(move |_| match backend.query(q) {
                    Ok(res) => {
                        h.handle(res);
                    }
                    Err(e) => {
                        println!("error: {}", e);
                    }
                });
            }
        })
        .unwrap();
    }
}
