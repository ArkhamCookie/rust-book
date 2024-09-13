fn main() {
	println!("--- Iterators ---");

	let v1 = vec![1, 2, 3];
	let v1_iter = v1.iter();

	for val in v1_iter {
		println!("Got: {val}");
	}

	println!("--- Methods that Produce Other Iterators ---");

	let v2: Vec<i32> = vec![1, 2, 3];
	let v3: Vec<_> = v2.iter().map(|x| x + 1).collect();

	assert_eq!(v3, vec![2, 3, 4]);
}
