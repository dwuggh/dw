mod server;
mod cli;

use server::Query;
use clap::{App, Arg};
use cli::formatter::format_ansi_term;
use std::fs::File;
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
        .get_matches();

    // load config
    let config = cli::config::read_config();
    let runner = cli::runner::Runner::new(config);

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

    let query = Arc::new(Query::new(&text, "en", "zh", false));
    let result = runner.run(query);
    for res in result {
        let format_string = format_ansi_term(&res);
        println!("{}", format_string);
    }
    Ok(())
}
