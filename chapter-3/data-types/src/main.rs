fn main() {
    let unsigned_int: u32 = 5;
    let signed_int: i32 = -2;

    println!("{unsigned_int}");
    println!("{signed_int}");

    // Numeric Operations
    println!("--- Numeric Operations ---");

    let sum = 5 + 10;   
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3;
    let remainder = 43 % 5;

    println!("sum = {sum}");
    println!("difference = {difference}");
    println!("product = {product}");
    println!("quotient = {quotient}");
    println!("truncated = {truncated}");
    println!("remainder = {remainder}");

    let t = true;
    let f: bool = false; // with explicit type annotation

    println!("{t}\n{f}");

    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    println!("{c} {z} {heart_eyed_cat}");
}