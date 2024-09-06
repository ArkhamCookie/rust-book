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
}
