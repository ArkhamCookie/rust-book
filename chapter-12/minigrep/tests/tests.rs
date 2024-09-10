#[cfg(test)]
mod tests {
	use minigrep::*;

	#[test]
	fn one_result() {
		let query = "duct";
		let contents: &str = "\
Rust:
safe, fast, productive.
Pick three.";
		
		assert_eq!(vec!["safe, fast, productive."], search(query, contents));
	}

	#[test]
	fn multible_results() {
		let query = "e";
		let contents = "\
Rust:
safe, fast, productive.
Pick three.";

		assert_eq!(vec!["safe, fast, productive.", "Pick three."], search(query, contents));
	}

	#[test]
	fn case_sensitive() {
		let query = "duct";
		let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

		assert_eq!(
			vec!["safe, fast, productive."],
			search(query, contents)
		);
	}

	#[test]
	fn case_insensitive() {
		let query = "rUsT";
		let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

		assert_eq!(
			vec!["Rust:", "Trust me."],
			search_case_insensitive(query, contents)
		);
	}
}
