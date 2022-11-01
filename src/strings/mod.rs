// https://doc.rust-lang.org/std/primitive.str.html#method.parse
// Because parse is so general, it can cause problems with type inference.
// As such, parse is one of the few times you’ll see the syntax affectionately
// known as the ‘turbofish’: ::<>. This helps the inference algorithm 
// understand specifically which type you’re trying to parse into.

mod strings {
    pub fn default() {
        let s: &str;
        s.split(",").map(|x| x.into()).collect::<Vec<String>>().into()
    }
}

fn main() {
    let four = "4".parse::<u32>().unwrap();
    assert_eq!(4, four);

    // strings::default();
}
