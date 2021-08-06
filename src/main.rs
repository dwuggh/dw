mod backends;
mod cli;
mod config;
mod formatter;
mod history;
pub mod runner;
mod transformer;
pub mod types;
pub use types::*;
pub mod server;

use cli::{build_cli, build_completion};
use formatter::Format;
use history::History;
use std::fs::File;
use std::io::prelude::Read;
use transformer::identify_language;
use transformer::{Concat, Transformer};

use server::{init_server, Params};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let matches = build_cli().get_matches();

    log::debug!("get clap matches: {:?}", matches);

    // info section
    if let Some(shell) = matches.value_of("generate-shell-completion") {
        build_completion(shell);
        return Ok(());
    }

    if matches.is_present("lang-codes") {
        let lang_codes = include_str!("lang_code.md");
        println!("{}\n", lang_codes);
        return Ok(());
    }

    // load config
    // TODO better error handling
    config::init().unwrap();

    // server
    if matches.is_present("server") {
        let addr = config::get().server.clone().unwrap().addr;
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
    let lang_from = match matches.value_of("lang-origin") {
        Some(lang) => lang,
        None => identify_language(&text),
    };
    let lang_to = match matches.value_of("lang-target") {
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

    let format: Format = matches.value_of("format").unwrap().into();

    let addr = config::get().server.clone().unwrap().addr;
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
        let params = Params::new(query, format);
        let res = client
            .post(format!("http://{}/lookup", addr))
            .json(&params)
            .send()
            .await
            .unwrap();
        let res = res.json::<String>().await.unwrap();
        println!("{}", res);
    } else {
        let runner = runner::Runner::new();
        let mut history = History::new();

        if query.is_short_text {
            history.add(&query.text, &query.lang_from);
        }
        let mut rx = runner.run(query, format).await;

        while let Some(text) = rx.recv().await {
            println!("\n\n{}", text);
        }
        history.dump()?;
    }

    Ok(())
}
