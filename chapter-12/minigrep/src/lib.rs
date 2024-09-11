pub mod search;

use std::error::Error;
use std::fs;
use std::path;

use clap::{ArgAction, Parser};

#[derive(Parser)]
pub struct Cli {
	pub query: String,
	pub file_path: path::PathBuf,

	#[arg(short, long, action = ArgAction::SetTrue)]
	pub ignore_case: bool,
	#[arg(short, long, action = ArgAction::SetTrue)]
	pub verbose: bool,
}

pub fn run(cli: Cli) -> Result<(), Box<dyn Error>> {
	if cli.verbose {
		println!("Querying for '{}' in `{:?}`.", cli.query, cli.file_path);
	}

	let contents = fs::read_to_string(cli.file_path).expect("Should be able to read this file");

	let results = if cli.ignore_case {
		search::case_insensitive(&cli.query, &contents)
	} else {
		search::default(&cli.query, &contents)
	};

	for line in results {
		println!("{line}");
	}

	Ok(())
}
