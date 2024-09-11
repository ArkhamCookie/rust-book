use std::io;

fn main() {
	println!("--- CONTROL FLOW ---");

	println!("--- `if` Expressions ---");

	println!("Enter a number...");

	let mut number = String::new();
	io::stdin()
		.read_line(&mut number)
		.expect("Failed to read line");

	let number: u32 = match number.trim().parse() {
		Ok(num) => num,
		Err(_) => todo!(),
	};

	if number < 5 {
		println!("Input is less than 5");
	} else if number > 5 {
		println!("Input is greater than 5")
	} else {
		println!("Input is not greater than or less than 5");
	}

	if number != 0 {
		println!("number was something other than zero");
	}

	if number % 4 == 0 {
		println!("number is divisible by 4");
	} else if number % 3 == 0 {
		println!("number is divisible by 3");
	} else if number % 2 == 0 {
		println!("number is divisible by 2");
	} else {
		println!("number is not divisible by 4, 3, or 2");
	}

	// Repetition with Loops
	println!("--- Repetition with Loops ---");

	let mut counter = 0;

	let result = loop {
		counter += 1;

		if counter == 10 {
			break counter * 2;
		}
	};

	println!("The result is {result}");

	let mut count = 0;

	'counting_up: loop {
		println!("count = {count}");
		let mut remaining = 10;

		loop {
			println!("remaining = {remaining}");
			if remaining == 9 {
				break;
			}

			if count == 2 {
				break 'counting_up;
			}
			remaining -= 1;
		}
		count += 1;
	}

	println!("End count = {count}");

	let mut number = 3;

	while number != 0 {
		println!("{number}!");

		number -= 1;
	}

	println!("LIFTOFF!!!");

	let a = [10, 20, 30, 40, 50];
	let mut index = 0;

	while index < 5 {
		println!("the value is: {}", a[index]);

		index += 1;
	}

	for element in a {
		println!("the value is: {element}");
	}

	for number in (1..4).rev() {
		println!("{number}!");
	}

	println!("LIFTOFF!!!");
}
