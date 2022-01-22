use clap_complete::generate_to;

include!("src/cli.rs");

fn main() {
    let mut app = build_cli();

    app.set_bin_name("dw");
    let outdir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("completions/");
    generate_to(Shell::Bash, &mut app, "dw", &outdir).unwrap();
    generate_to(Shell::Zsh, &mut app, "dw", &outdir).unwrap();
    generate_to(Shell::PowerShell, &mut app, "dw", &outdir).unwrap();
    generate_to(Shell::Fish, &mut app, "dw", &outdir).unwrap();
    generate_to(Shell::Elvish, &mut app, "dw", &outdir).unwrap();
}
