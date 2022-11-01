// https://github.com/timvisee/advent-of-code-2020
// https://doc.rust-lang.org/rust-by-example/index.html

use std::convert::TryInto;

// find error in the code
#[allow(dead_code)]
pub fn plus_five(value: i32) -> isize {
    (value + 1).try_into().unwrap()
}
