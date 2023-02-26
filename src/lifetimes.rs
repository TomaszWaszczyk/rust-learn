fn main() {
}

// 1. Each parameter that is a reference gets its own lifetime parameter
// 2. If there is exactly one input lifetime parameter, that lifetime parameter is assigned to all outputs lifetime parameters;
// 3. If there are multiple input lifetime parameters, but one of them is &self or &mut self the lifetime of self is assigned to all output lifetime parameters.

fn first_word<'lifetime>(s: &'lifetime str) -> &'lifetime str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
