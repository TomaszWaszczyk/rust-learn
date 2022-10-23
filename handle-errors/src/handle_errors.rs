use std::io;
use std::process;

pub fn handle_errors() {
    println!("Please enter a first number: ");

    let mut first = String::new();
    io::stdin().read_line(&mut first).unwrap(); // `unwrap` is good for dev, not prod!

    let mut a: u32 = first.trim().parse().expect("This is not a valid number"); // TO_LEARN: unwrap() vs expect()

    match first.trim().parse() {
        Ok(val) => {
            a = val;
        }
        Err(_err) => {
            println!("This is not a valid number");
            process::exit(1); // exit with non zero value === error
        }
    };

    println!("Please enter a second number: ");

    let mut second = String::new();
    io::stdin().read_line(&mut second);

    let mut b: u32 = second.trim().parse().expect("This is not a valid number");
    match second.trim().parse() {
        Ok(val) => {
            a = val;
        }
        Err(_err) => {
            println!("This is not a valid number");
        }
    };
}
