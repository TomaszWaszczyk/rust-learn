use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Hello, it is a guess game!");
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your number...");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Failed to read line");

        let guess = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input a number.");
                continue;
            }
        }
    }
}
