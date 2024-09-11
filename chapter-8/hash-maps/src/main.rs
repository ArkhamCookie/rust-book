fn main() {
	println!("--- Hash Maps ---");

	use std::collections::HashMap;

	let mut scores = HashMap::new();

	scores.insert(String::from("Blue"), 10);
	scores.insert(String::from("Yellow"), 50);

	println!("{scores:?}");

	let team_name = String::from("Blue");
	let score = scores.get(&team_name).copied().unwrap_or(0);

	println!("{score}");

	for (key, value) in &scores {
		println!("{key}: {value}");
	}

	// Overwriting/Updating a Value
	scores.insert(String::from("Blue"), 25);

	println!("{scores:?}");

	// Only updates if value doesn't exist
	scores.entry(String::from("Green")).or_insert(50);
	scores.entry(String::from("Blue")).or_insert(50);

	println!("{scores:?}");

	let text = "hello world wonderful world";

	let mut map = HashMap::new();

	for word in text.split_whitespace() {
		let count = map.entry(word).or_insert(0);
		*count += 1;
	}

	println!("{map:?}");
}
