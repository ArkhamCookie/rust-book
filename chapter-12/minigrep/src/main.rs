use std::process;
use clap::Parser;
use minigrep::{run, Cli};

fn main() {
    let args = Cli::parse();

    if let Err(e) = run(args) {
        eprint!("Application error: {e}");
        process::exit(1);
    }
}
