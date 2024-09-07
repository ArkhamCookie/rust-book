#![allow(dead_code)]

fn main() {
    println!("--- Defining Enums ---");

    #[derive(Debug)]
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    impl Message {
        fn call(&self) {
            let output = &self;

            println!("{output:?}")
        }
    }

    let mut message;

    message = Message::Write(String::from("Hello"));
    message.call();

    message = Message::Quit;
    message.call();

    message = Message::Move{x: 30, y: 10};
    message.call();

    message = Message::ChangeColor(0, 2, 5);
    message.call();

    println!("--- Option Emnu ---");

    let character = Some('e');
    let absent_number: Option<i32> = None;

    println!("{character:?}");
    println!("{absent_number:?}");
}
