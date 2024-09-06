fn another_function(x: i32) {
	println!("The value of x is: {x}");
}

fn statements_expressions() {
	let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}

// Functions can be defined either before or after the main function.
fn main() {
	another_function(5);
	print_labeled_measurement(5, 'h');

	println!("--- Statements and Expressions ---");
	statements_expressions();
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}
