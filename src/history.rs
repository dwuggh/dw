use chrono::prelude::*;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use xdg::BaseDirectories;

pub struct History {
    file: PathBuf,
    items: Vec<HistoryItem>,
}

#[derive(Clone, Debug)]
struct HistoryItem {
    time: DateTime<Utc>,
    text: String,
    lang: String,
}

impl HistoryItem {
    pub fn new(text: &str, lang: &str) -> HistoryItem {
        HistoryItem {
            time: chrono::offset::Utc::now(),
            text: text.to_string(),
            lang: lang.to_string(),
        }
    }
}

impl History {
    pub fn new() -> Self {
        let dir = BaseDirectories::with_prefix("dw").unwrap();
        let mut h = History {
            file: dir.place_data_file("dw.history").unwrap(),
            items: Vec::new(),
        };
        h.load().unwrap();
        h
    }

    pub fn load(&mut self) -> std::io::Result<()> {
        if self.file.is_file() {
            let mut file = File::open(self.file.to_str().unwrap())?;
            log::info!("loading history from file: {:?}", self.file);
            let mut buf = String::new();
            file.read_to_string(&mut buf)?;
            let items: Vec<HistoryItem> = buf
                .split_terminator('\n')
                .map(|line| {
                    let a: Vec<&str> = line.splitn(3, ' ').collect();
                    let item = HistoryItem {
                        time: DateTime::parse_from_rfc3339(a[0]).unwrap().into(),
                        text: a[2].to_string(),
                        lang: a[1].to_string(),
                    };
                    log::debug!("loaded history item: {:?}", item);
                    item
                })
                .collect();
            self.items = items;
        } else {
        }
        Ok(())
    }

    pub fn add(&mut self, text: &str, lang: &str) {
        self.items.push(HistoryItem::new(text, lang));
    }

    pub fn dump(&self) -> std::io::Result<()> {
        let mut f = File::create(&self.file)?;
        for item in &self.items {
            writeln!(
                &mut f,
                "{} {} {}",
                item.time.to_rfc3339(),
                item.lang,
                &item.text
            )?;
        }
        Ok(())
    }
}
