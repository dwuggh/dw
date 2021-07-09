use std::rc::Rc;
use std::sync::Arc;

use super::backends::google_translate::GTrans;
use super::backends::youdao::Youdao;
use super::config::ConfigRef;
use super::formatter::Formatter;
use super::{Backend, Query, RespData};
use crossbeam::thread;

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
