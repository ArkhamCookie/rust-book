use std::{thread, time::Duration};

#[derive(Debug, Clone, Copy)]
enum ShirtColor {
	Red,
	Blue,
}

struct Inventory {
	shirts: Vec<ShirtColor>,
}

#[derive(Debug)]
struct Rectangle {
	width: u32,
	height: u32,
}

impl Inventory {
	fn most_stocked(&self) -> ShirtColor {
		let mut num_red = 0;
		let mut num_blue = 0;

		for color in &self.shirts {
			match color {
				ShirtColor::Red => num_red += 1,
				ShirtColor::Blue => num_blue += 1,
			}
		}

		if num_red > num_blue {
			ShirtColor::Red
		} else {
			ShirtColor::Blue
		}
	}

	fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
		user_preference.unwrap_or_else(|| self.most_stocked())
	}
}

fn generate_workout(intensity: u32, random_number: u32) {
	let expensive_closure = |num: u32| -> u32 {
		println!("calculating slowly...");
		thread::sleep(Duration::from_secs(2));
		num
	};

	if intensity < 25 {
		println!("Today, I do {} pushups!", expensive_closure(intensity));
		println!("Next, do {} situps!", expensive_closure(intensity));
	} else {
		if random_number == 3 {
			println!("Take a break today! Remember to stay hydrated!");
		} else {
			println!("Today, run for {} minutes!", expensive_closure(intensity));
		}
	}
}

fn main() {
	println!("--- Closures ---");

	let store = Inventory {
		shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
	};
	let user_pref1 = Some(ShirtColor::Red);
	let giveaway1 = store.giveaway(user_pref1);

	println!(
		"The user with preference {:?} gets {:?}",
		user_pref1, giveaway1
	);

	let user_pref2 = None;
	let giveaway2 = store.giveaway(user_pref2);

	println!(
		"The user with preference {:?} gets {:?}",
		user_pref2, giveaway2
	);

	println!("--- Closure Type Inference and Annotation ---");

	let simulated_user_specified_value = 10;
	let simulated_random_number = 7;

	generate_workout(simulated_user_specified_value, simulated_random_number);

	println!("--- Capturing References or Moving Ownership ---");

	let list1 = vec![1, 2, 3];

	println!("Before defining closure: {list1:?}");

	let only_borrows = || println!("From closure: {list1:?}");

	println!("Before calling closure: {list1:?}");
	only_borrows();
	println!("After calling closure: {list1:?}");

	let mut list2 = vec![1, 2, 3];

	println!("Before defining closure: {list2:?}");

	let mut borrows_mutably = || list2.push(7);

	borrows_mutably();
	println!("After calling closure: {list2:?}");

	let list3 = vec![1, 2, 3];

	println!("Before defining closure: {list3:?}");

	thread::spawn(move || println!("From thread: {list3:?}"))
		.join()
		.unwrap();

	println!("--- Moving Captured Values Out of Closures and the `Fn` Traits ---");

	let mut list4 = [
		Rectangle {
			width: 10,
			height: 1,
		},
		Rectangle {
			width: 3,
			height: 5,
		},
		Rectangle {
			width: 7,
			height: 12,
		},
	];

	list4.sort_by_key(|r| r.width);

	println!("{list4:#?}");

	let mut list5 = [
		Rectangle {
			width: 10,
			height: 1,
		},
		Rectangle {
			width: 3,
			height: 5,
		},
		Rectangle {
			width: 7,
			height: 12,
		},
	];
	let mut num_sort_operations = 0;

	list5.sort_by_key(|r| {
		num_sort_operations += 1;
		r.width
	});

	println!("{list5:#?}, sorted in {num_sort_operations} operations.");
}
