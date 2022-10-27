use std::convert::TryInto;

// find error in the code
pub fn plus_five(value: i32) -> isize {
    (value + 1).try_into().unwrap()
}
