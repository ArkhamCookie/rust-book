fn main() {
	println!("--- Strings ---");

	let s = String::new();

	println!("{s}");

	let data = "intial contents";
	let mut s = data.to_string();

	println!("{s}");

	s = "intial contents".to_string();

	println!("{s}");

	s = String::from("foo");

	s.push_str("bar");

	println!("{s}");

	s = String::from("lo");

	// Can only push one character
	s.push('l');

	println!("{s}");

	let s1 = String::from("tic");
	let s2 = String::from("tac");
	let s3 = String::from("toe");

	s = format!("{s1}-{s2}-{s3}");

	println!("{s}");

	for c in "Зд".chars() {
		println!("{c}");
	}

	for b in "Зд".bytes() {
		println!("{b}");
	}
}
