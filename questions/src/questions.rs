mod questions;


fn questions() {
    balanced("Test");
}

/// O(n)
/// Example of pattern matching and documentation a'la Rust lang
/// ```
/// balanced("pass your string here");
/// ```
fn balanced(input: &str) -> bool {

    if input.len() == 1 {
        return false;
    }

    let mut stack: Vec<char> = vec![];
    for c in input.chars() {
        match c {
            '(' | '[' | '{' => stack.push(c),
            ')' | ']' | '}' => match (stack.pop(), c) {
                ( Some('('), ')' ) => {}
                ( Some('['), ']' ) => {}
                ( Some('{'), '}' ) => {}
                // You could just use `_ => return false` instead of `(_, _) => ...`
                (_, _) => return false,
            },
            _ => {}
        }
    }
    // One slight improvement is using `.is_empty()` instead of `.len() == 0`
    stack.len() == 0
}
