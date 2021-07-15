mod server;

use clap::{App, Arg};
use server::formatter::Formatter;
use server::transformer::identify_language;
use server::transformer::{Concat, Transformer};
use server::{History, Query};
use std::fs::File;
use std::{io::prelude::*, sync::Arc};

use crate::server::{init_server, Params};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let matches = App::new("dw")
        .version("0.2.0")
        .author("dwuggh <dwuggh@gmail.com>")
        .about("A simple dictionary wrapper.")
        .arg(
            Arg::new("server")
                .about("server mode")
                .long("server")
                .takes_value(false),
        )
        .arg(
            Arg::new("standalone")
                .about("standalone client mode")
                .long("standalone")
                .takes_value(false),
        )
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
    // TODO better error handling
    server::config::init().unwrap();

    // server
    if matches.is_present("server") {
        let addr = server::config::get().server.clone().unwrap().addr;
        log::info!("initializing server on {}", addr);
        return init_server(&addr).await;
    }

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
    text = Concat::default().act(&text);
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
    let query = Query::new(&text, lang_from, lang_to, false);
    let addr = server::config::get().server.clone().unwrap().addr;

    let server_is_ready = reqwest::Client::new()
        .get(&addr)
        .send()
        .await
        .ok()
        .and_then(|a| a.status().is_success().then(|| 0))
        .is_some();

    if !matches.is_present("standalone") && server_is_ready {
        log::info!("using server to get response");
        let client = reqwest::Client::new();
        let params = Params::new(query, Formatter::AnsiTerm);
        let res = client
            .post(format!("http://{}/lookup", addr))
            .json(&params)
            .send()
            .await
            .unwrap();
        let res = res.json::<String>().await.unwrap();
        println!("{}", res);
    } else {
        let runner = server::runner::Runner::new();
        let mut history = History::new();

        if query.is_short_text {
            history.add(&query.text, &query.lang_from);
        }
        let mut rx = runner.run(Arc::new(query), Formatter::AnsiTerm).await;

        while let Some(text) = rx.recv().await {
            println!("\n\n{}", text);
        }
        history.dump()?;
    }

    Ok(())
}
