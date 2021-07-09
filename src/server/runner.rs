
use std::sync::Arc;

use super::backends::google_translate::GTrans;
use super::backends::youdao::Youdao;
use super::formatter::Formatter;
use super::{Backend, Query};
use crossbeam::thread;

pub struct Runner {
    backends: Vec<Box<dyn Backend>>,
}

impl Runner {
    pub fn new() -> Self {
        let mut backends: Vec<Box<dyn Backend>> = Vec::new();
        backends.push(Box::new(GTrans::new()));
        backends.push(Box::new(Youdao::new()));

        Runner { backends }
    }

    pub fn run(&self, query: Arc<Query>, formatter: Formatter) {
        thread::scope(|s| {
            for backend in &self.backends {
                let q = Arc::clone(&query);
                s.spawn(move |_| match backend.query(q) {
                    Ok(res) => {
                        let str = formatter.format(&res);
                        print!("{}", str);
                    }
                    Err(e) => {
                        eprintln!("{}", e);
                    }
                });
            }
        })
        .expect("server::runner::Runner::run error");
    }
}
