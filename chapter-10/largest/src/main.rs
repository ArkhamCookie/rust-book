fn main() {
	println!("--- Largest ---");

	fn get_largest(list: &[i32]) -> &i32 {
		let mut largest = &list[0];

		for number in list {
			if number > largest {
				largest = number;
			}
		}

		largest
	}

	let number_list = vec![34, 50, 25, 100, 65];

	let result = get_largest(&number_list);
	println!("The largest number is {result}!");

	let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

	let result = get_largest(&number_list);
	println!("The largest number is {result}!");
}
