use clap_generate::generate_to;

include!("src/cli.rs");

fn main() {
    let mut app = build_cli();

    app.set_bin_name("dw");
    let outdir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("completions/");
    generate_to::<Bash, _, _>(&mut app, "dw", &outdir);
    generate_to::<Zsh, _, _>(&mut app, "dw", &outdir);
    generate_to::<PowerShell, _, _>(&mut app, "dw", &outdir);
    generate_to::<Elvish, _, _>(&mut app, "dw", &outdir);
    generate_to::<Fish, _, _>(&mut app, "dw", &outdir);
}
