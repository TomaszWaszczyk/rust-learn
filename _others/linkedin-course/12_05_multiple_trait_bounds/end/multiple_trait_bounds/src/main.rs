use std::fmt;

// fn compare_and_print<T: fmt::Display + PartialEq + From<U>, U: fmt::Display + PartialEq>(a: T, b: U) {
fn compare_and_print<T, U>(a: T, b: U) 
    where T: fmt::Display + PartialEq + From<U>,
          U: fmt::Display + PartialEq 
{
        if a == T::from(b) {
        println!("{} is equal to {}", a, b);
    } else {
        println!("{} is NOT equal to {}", a, b);
    }
}

fn main() {
    compare_and_print(1.0, "one");
    compare_and_print(1.1, 1);
}