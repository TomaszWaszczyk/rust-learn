fn main() {
    let countdown = [5, 4, 3, 2, 1];
    let number = countdown.get(5);
    let number = number.unwrap_or(&0) + 1;
    println!("number is {:?}", number);
}