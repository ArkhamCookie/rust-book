struct User {
	active: bool,
	username: String,
	email: String,
	sign_in_count: u64,
}

#[derive(Debug)]
struct Rectangle {
	width: u32,
	height: u32,
}

impl Rectangle {
	fn area(&self) -> u32 {
		return self.width * self.height;
	}

	fn set_width(&mut self, width: u32) {
		self.width = width;
	}
}

fn build_user(email: String, username: String) -> User {
	User {
		active: true,
		username,
		email,
		sign_in_count: 0,
	}
}

fn get_area(rectangle: &Rectangle) -> u32 {
	rectangle.width * rectangle.height
}

fn main() {
	println!("--- Structs ---");

	let mut email: String;
	let mut username: String;
	let mut active: bool;
	let mut sign_in_count: u64;

	let user1 = User {
		email: String::from("someone@example.com"),
		username: String::from("someusername"),
		active: true,
		sign_in_count: 1,
	};

	email = user1.email;
	username = user1.username;
	active = user1.active;
	sign_in_count = user1.sign_in_count;

	if active {
		println!("{username} ({email}) is active and has signed in {sign_in_count} time(s).");
	} else {
		println!("{username} ({email}) is not active.");
	}

	let user2 = build_user(
		"someone-else".to_string(),
		"someone-else@example.com".to_string(),
	);

	email = user2.email;
	username = user2.username;
	active = user2.active;
	sign_in_count = user2.sign_in_count;

	if active {
		println!("{username} ({email}) is active and has signed in {sign_in_count} time(s).");
	} else {
		println!("{username} ({email}) is not active.");
	}

	println!("\n--- Area Example ---");

	let rectangle1 = Rectangle {
		width: 3,
		height: 5,
	};
	let rectangle2 = Rectangle {
		width: 5,
		height: 5,
	};
	let mut rectangle3 = Rectangle {
		width: 1,
		height: 5,
	};

	let mut area: u32;

	area = get_area(&rectangle1);

	println!("rectangle1 is {rectangle1:?}");
	println!("The area of the rectangle1 is {area}");

	area = get_area(&rectangle2);

	dbg!(&rectangle2);
	println!("The area of the rectangle2 is {area}");

	rectangle3.set_width(2);
	area = rectangle3.area();

	println!("The area of rectangle3 is {area}");
}
