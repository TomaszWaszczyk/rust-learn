// https://stackoverflow.com/questions/28621980/what-are-the-actual-runtime-performance-costs-of-dynamic-dispatch

// pub fn demo(x: impl Iterator<Item = i32>) -> i32 {
//     x.sum()
// }

// fn main() {
//     let it = 0..10;
//     assert_eq!(demo(it), 45); // static dispatch
    
//     let mut it = 0..10;
//     let it: &mut dyn Iterator<Item = i32> = &mut it;
//     assert_eq!(demo(it), 45); // dynamic dispatch
// }
// ================================================================ //

pub trait Hey {
    fn hey(&self);
}

impl Hey for &str {
    fn hey(&self) {
        println!("hey {}", self);
    }
}

pub fn foo() {
    "J".hey();
}

pub fn bar(h: impl Hey) {
    h.hey(); // no information about "h" type here -> static dispatch
}
