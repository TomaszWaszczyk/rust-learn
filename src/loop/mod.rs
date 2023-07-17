fn main() {
    let i: i32 = 1;
    let result = loop {
        i += 1;
        if i == 11 {
            break i;
        }
    };

    println!("{}", result);
}
