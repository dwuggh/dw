use clap::{App, Arg};
use clap_generate::generators::{Bash, Elvish, Fish, PowerShell, Zsh};
use clap_generate::{generate, Generator};

pub fn build_cli() -> App<'static> {
    App::new("dw")
        .version("0.2.1")
        .author("dwuggh <dwuggh@gmail.com>")
        .about("A simple dictionary wrapper.")
        .arg(
            Arg::new("generate-shell-completion")
                .about("generate shell completion")
                .long("generate-shell-completion")
                .possible_values(&["bash", "zsh", "powershell", "fish", "elvish"])
                .takes_value(true),
        )
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
        .arg(
            Arg::new("INPUT")
                .about("input")
                .required(false)
                .multiple(true),
        )
        .arg(
            Arg::new("file")
                .about("use file")
                .short('f')
                .long("file")
                .takes_value(true),
        )
        .arg(
            Arg::new("lang-origin")
                .about("origin language of the querying text")
                .short('o')
                .long("lang-origin")
                .takes_value(true),
        )
        .arg(
            Arg::new("lang-target")
                .about("the language to be translated into")
                .short('t')
                .long("lang-target")
                .takes_value(true),
        )
        .arg(
            Arg::new("lang-codes")
                .about("display all available language codes")
                .long("lang-code"),
        )
        .arg(
            Arg::new("format")
                .about("response format")
                .long("format")
                .possible_values(&["md", "ansi"])
                .default_value("ansi"),
        )
}

pub fn print_completions<G: Generator>(app: &mut App) {
    generate::<G, _>(app, app.get_name().to_string(), &mut std::io::stdout());
}

pub fn build_completion(shell: &str) {
    let mut app = build_cli();
    match shell {
        "bash" => print_completions::<Bash>(&mut app),
        "elvish" => print_completions::<Elvish>(&mut app),
        "fish" => print_completions::<Fish>(&mut app),
        "powershell" => print_completions::<PowerShell>(&mut app),
        "zsh" => print_completions::<Zsh>(&mut app),
        _ => {
            eprintln!("unknown generator: {}", shell);
        }
    }
}
