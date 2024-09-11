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
