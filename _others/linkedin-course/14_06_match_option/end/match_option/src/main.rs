fn main() {
    let countdown = [5, 4, 3, 2, 1];
    let number = countdown.get(2);
    let number = match number {
        Some(number) => number + 1,
        None => 0
    };
    println!("number is {:?}", number);
}