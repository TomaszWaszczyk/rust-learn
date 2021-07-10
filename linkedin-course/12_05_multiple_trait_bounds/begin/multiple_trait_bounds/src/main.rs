fn compare_and_print<T, U>(a: T, b: U) {
    if a == T::from(b) {
        println!("{} is equal to {}", a, b);
    } else {
        println!("{} is NOT equal to {}", a, b);
    }
}

fn main() {
    compare_and_print(1.0, 1);
    compare_and_print(1.1, 1);
}