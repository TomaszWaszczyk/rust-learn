fn main() {
    //panic!("Houston, we've had a problem.");

    let countdown = [5, 4, 3, 2, 1, 0];

    for count in countdown.iter() {
        println!{"T-minus {}", count};
        let x = 1 / count; // this won't end well
    }
}
