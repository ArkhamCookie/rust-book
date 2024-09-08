fn main() {
    println!("--- Vectors ---");

    let mut v: Vec<i32> = Vec::new();

    println!("{v:?}");

    v = vec![1, 2, 3];

    println!("{v:?}");

    v.push(5);
    v.push(5);
    v.push(6);
    v.push(7);

    println!("{v:?}");

    // Best if we want our program to crash if third doesn't exist
    let third: &i32 = &v[2];

    println!("{third:?}");

    // Better in most cases
    let third: Option<&i32> = v.get(2);

    match third {
        Some(third) => println!("The third element is {third}."),
        None => println!("There is no third element."),
    }

    let v = vec![100, 32, 57];

    for n_ref in &v {
        let n_plus_one: i32 = *n_ref + 1;

        println!("{n_plus_one}");
    }

    let mut v = vec![100, 32, 57];

    for n_ref in &mut v {
        *n_ref += 50;
    }

    #[allow(dead_code)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let _row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
