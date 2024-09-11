#[cfg(test)]
use std::error;
use std::process::Command;

use assert_cmd::prelude::*;
use predicates::prelude::predicate;

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn error::Error>> {
	let mut cmd = Command::cargo_bin("minigrep")?;

	cmd.arg("foobar").arg("tests/data/doesnt-exist");
	cmd.assert()
		.failure()
		.stderr(predicate::str::contains("Should be able to read this file"));

	Ok(())
}

#[test]
fn find_content_in_file() -> Result<(), Box<dyn error::Error>> {
	let file = "tests/data/poem.txt";
	let mut cmd = Command::cargo_bin("minigrep")?;

	cmd.arg("frog").arg(file);

	cmd.assert().success().stdout("How public, like a frog\n");

	Ok(())
}

#[test]
fn no_match_in_file() -> Result<(), Box<dyn error::Error>> {
	let file = "tests/data/poem.txt";
	let mut cmd = Command::cargo_bin("minigrep")?;

	cmd.arg("Frog").arg(file);

	cmd.assert().success().stdout("");

	Ok(())
}
