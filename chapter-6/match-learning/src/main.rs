#![allow(dead_code)]

use rand::Rng;

enum Coin {
	Penny,
	Nickel,
	Dime,
	Quarter(USState),
}

#[derive(Debug)]
enum USState {
	Alabama,
	Alaska,
	// Rest of the states...
}

fn main() {
	println!("--- `match` Control Flow Construct ---");

	fn value_in_cents(coin: Coin) -> u8 {
		match coin {
			Coin::Penny => {
				println!("Lucky penny!");
				1
			}
			Coin::Nickel => 5,
			Coin::Dime => 10,
			Coin::Quarter(state) => {
				println!("State quarter is from {state:?}.");
				25
			}
		}
	}

	let mut my_coin: Coin;
	let mut coin_value;

	my_coin = Coin::Penny;
	coin_value = value_in_cents(my_coin);

	println!("{coin_value}");

	my_coin = Coin::Quarter(USState::Alaska);
	coin_value = value_in_cents(my_coin);

	println!("{coin_value}");

	println!("--- Matching with `Option<T>` ---");

	fn plus_one(x: Option<i32>) -> Option<i32> {
		match x {
			None => None,
			Some(i) => Some(i + 1),
		}
	}

	let five = Some(5);
	let six = plus_one(five);
	let none = plus_one(None);

	println!("{six:?}");
	println!("{none:?}");

	println!("--- Catch-all Patterns ---");

	let mut dice_roll = rand::thread_rng().gen_range(1..=10);

	fn add_fancy_hat() {
		println!("Added fancy hat :)");
	}

	fn remove_fancy_hat() {
		println!("Removed fancy hat :(");
	}

	loop {
		println!("You rolled a {dice_roll}.");

		match dice_roll {
			3 => {
				add_fancy_hat();
				break;
			}
			7 => {
				remove_fancy_hat();
				break;
			}
			_ => dice_roll = rand::thread_rng().gen_range(1..=10),
		}
	}
}
