use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use xdg::BaseDirectories;

use super::config::Config;

pub struct History {
    file: PathBuf,
    content: String,
}

impl History {
    #[allow(dead_code)]
    pub fn new(_config: Config) -> Self {
        let dir = BaseDirectories::with_prefix("dw").unwrap();
        let mut h = History {
            file: dir.place_data_file("dw.history").unwrap(),
            content: String::new(),
        };
        h.load().unwrap();
        h
    }

    #[allow(dead_code)]
    pub fn load(&mut self) -> std::io::Result<()> {
        if self.file.is_file() {
            let mut file = File::open(self.file.to_str().unwrap())?;
            file.read_to_string(&mut self.content)?;
        } else {
        }
        Ok(())
    }
}
