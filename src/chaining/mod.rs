fn parse_int(value: &str) -> Option<i32> {
    value.parse().ok()
}

fn triple_num(value: i32) -> Option<i32> {
    Some(value * 5)
}

fn main() {
    let result = Some("20")
    .and_then(parse_int)
    .and_then(triple_num);

    match result {
        Some(value) => println!("Result: {}", value),
        None => println!("Invalid input!"),
    }
}
