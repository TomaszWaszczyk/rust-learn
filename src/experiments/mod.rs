use std::convert::TryInto;

// https://github.com/timvisee/advent-of-code-2020
// find error in the code
pub fn plus_five(value: i32) -> isize {
    (value + 1).try_into().unwrap()
}
