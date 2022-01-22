use clap::{App, Arg};
use clap_complete::{generate, Generator, Shell};

pub fn build_cli() -> App<'static> {
    App::new("dw")
        .version("0.2.1")
        .author("dwuggh <dwuggh@gmail.com>")
        .about("A simple dictionary wrapper.")
        .arg(
            Arg::new("generate-shell-completion")
                .help("generate shell completion")
                .long("generate-shell-completion")
                .possible_values(Shell::possible_values())
                .takes_value(true),
        )
        .arg(
            Arg::new("server")
                .help("server mode")
                .long("server")
                .takes_value(false),
        )
        .arg(
            Arg::new("standalone")
                .help("standalone client mode")
                .long("standalone")
                .takes_value(false),
        )
        .arg(
            Arg::new("INPUT")
                .help("input")
                .required(false)
                .multiple_values(true),
        )
        .arg(
            Arg::new("file")
                .help("use file")
                .short('f')
                .long("file")
                .takes_value(true),
        )
        .arg(
            Arg::new("lang-origin")
                .help("origin language of the querying text")
                .short('o')
                .long("lang-origin")
                .takes_value(true),
        )
        .arg(
            Arg::new("lang-target")
                .help("the language to be translated into")
                .short('t')
                .long("lang-target")
                .takes_value(true),
        )
        .arg(
            Arg::new("lang-codes")
                .help("display all available language codes")
                .long("lang-code"),
        )
        .arg(
            Arg::new("format")
                .help("response format")
                .long("format")
                .possible_values(&["md", "ansi"])
                .default_value("ansi"),
        )
}

pub fn print_completions<G: Generator>(gen: G, app: &mut App) {
    generate(gen, app, app.get_name().to_string(), &mut std::io::stdout());
}
