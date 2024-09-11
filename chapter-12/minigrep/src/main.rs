use std::process;

use minigrep::{run, Cli};

use clap::Parser;

fn main() {
	let args = Cli::parse();

	if let Err(e) = run(args) {
		eprint!("Application error: {e}");
		process::exit(1);
	}
}
