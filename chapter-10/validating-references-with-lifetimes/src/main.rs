fn main() {
    println!("--- Validating References with Lifetimes ---");

    let x = 5;
    let r = &x;

    println!("{x}, {r}");

    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    let string1 = String::from("abcd");
    let string2 = "xyz";

    println!("{}", longest(&string1, &string2));

    use std::fmt::Display;

    fn longest_with_an_announcement<'a, T>(
        x: &'a str,
        y: &'a str,
        ann: T,
    ) -> &'a str
    where
        T: Display,
    {
            println!("Announcement! {ann}");
            if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    let variable_name = longest_with_an_announcement("foo", "foobar", "Hello");
    println!("{variable_name}");
}
