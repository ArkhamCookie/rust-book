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
    pub verbose: bool,
    #[arg(short, long, action = ArgAction::SetTrue)]
    pub ignore_case: bool,
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

pub fn run(cli: Cli) -> Result<(), Box<dyn Error>> {
    if cli.verbose {
        println!("Querying for '{}' in `{:?}`.", cli.query, cli.file_path);
    }

    let contents = fs::read_to_string(cli.file_path).expect("Should be able to read this file");

    let results = if cli.ignore_case {
        search_case_insensitive(&cli.query, &contents)
    } else {
        search(&cli.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}
