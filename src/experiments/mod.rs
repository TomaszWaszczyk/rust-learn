// https://github.com/timvisee/advent-of-code-2020
// https://doc.rust-lang.org/rust-by-example/index.html

use std::convert::TryInto;

// find error in the code
#[allow(dead_code)]
pub fn plus_five(value: i32) -> isize {
    (value + 1).try_into().unwrap()
}

pub mod currying {
    // Currying transforms a function with multiple arguments into functions that only take one.
    // It can be achieved in Rust using closures because of their ability to capture variables 
    // from their defined scope. It's a pattern often used in functional programming 
    // but not one I have seen used much in Rust.

    // 👉 Currying can help when you have to call a function with a fixed argument frequently
    // 👉 It's a way to produce high-order functions that contain context 
    // 👉 Can make the function calls easier to read because of fewer arguments
    pub fn multiply_curry(number: i32) -> impl Fn(i32) -> i32 {
        move |x| number * x
    }
}

pub fn trim_extra_whitespace(item: &str) -> &str {
    if let Some(stripped) = item.strip_prefix(' ') {
        stripped.trim_end()
    } else {
        item.trim_end()
    }
}

// pub fn get_list(list: &mut Vec<String>) -> &mut String {
//     if let Some(s) = list.first_mut() {
//         return s;
//     }

//     list.push(format!("Hello, world!"));
//     list.first_mut().unwrap()
// }

fn main() {
    // let double = currying::multiply_curry(2);

    // let double_four = double(4);
    // let double_five = double(5);

    // assert_eq!(double_four, 8);
    // assert_eq!(double_five, 10);
    // ================================================================================================= //
    let prefix : &str = " he";
    let post = trim_extra_whitespace(prefix);

    // let mut vec: Vec<_> = Vec::new();

    assert_eq!(prefix, post);
}
