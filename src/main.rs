mod cli;
mod server;

use clap::{App, Arg};
use cli::formatter::AnsiTermHandler;
use server::transformer::identify_language;
use server::{History, Query};
use std::fs::File;
use std::rc::Rc;
use std::{io::prelude::*, sync::Arc};

fn main() -> std::io::Result<()> {
    env_logger::init();
    let matches = App::new("dw")
        .version("0.1.0")
        .author("dwuggh <dwuggh@gmail.com>")
        .about("dict wowo")
        .arg(Arg::new("INPUT").about("input").required(false).index(1))
        .arg(
            Arg::new("file")
                .about("use file")
                .short('f')
                .long("file")
                .takes_value(true),
        )
        .arg(
            Arg::new("lang_origin")
                .about("origin language of the querying text")
                .short('o')
                .long("lang-origin"),
        )
        .arg(
            Arg::new("lang_target")
                .about("the language to be translated into")
                .short('t')
                .long("lang-target"),
        )
        .get_matches();

    log::debug!("get clap matches: {:?}", matches);

    // load config
    let config = Rc::new(server::config::read_config());
    let runner = server::runner::Runner::new(Rc::clone(&config));
    let mut history = History::new(Rc::clone(&config));

    let mut text = String::new();
    if let Some(file) = matches.value_of("file") {
        let mut f = File::open(file)?;
        f.read_to_string(&mut text)?;
    } else if let Some(_text) = matches.value_of("INPUT") {
        text = _text.to_string();
    } else {
        std::io::stdin().read_to_string(&mut text)?;
    }
    if text.is_empty() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "query string is empty",
        ));
    }
    log::info!("query string: {}", text);
    let lang_from = match matches.value_of("lang_origin") {
        Some(lang) => lang,
        None => identify_language(&text),
    };
    let lang_to = match matches.value_of("lang_target") {
        Some(lang) => lang,
        None => {
            if lang_from == "zh" {
                "en"
            } else {
                "zh"
            }
        }
    };

    let query = Arc::new(Query::new(&text, lang_from, lang_to, false));
    if query.is_short_text {
        history.add(&query.text, &query.lang_from);
    }
    runner.run(query, Arc::new(AnsiTermHandler));
    history.dump()?;
    Ok(())
}
