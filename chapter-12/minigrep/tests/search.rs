#[cfg(test)]
use minigrep::*;

const CONTENT: &str = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.
Trust me.";

#[test]
fn one_result() {
	let query = "three";
	let contents = CONTENT;

	assert_eq!(vec!["Pick three."], search(query, contents));
}

#[test]
fn multible_results() {
	let query = "e";
	let contents = CONTENT;

	assert_eq!(
		vec![
			"safe, fast, productive.",
			"Pick three.",
			"Duct tape.",
			"Trust me."
		],
		search(query, contents)
	);
}

#[test]
fn case_sensitive() {
	let query = "duct";
	let contents = CONTENT;

	assert_eq!(vec!["safe, fast, productive."], search(query, contents));
}

#[test]
fn case_insensitive() {
	let query = "rUsT";
	let contents = CONTENT;

	assert_eq!(
		vec!["Rust:", "Trust me."],
		search_case_insensitive(query, contents)
	);
}
