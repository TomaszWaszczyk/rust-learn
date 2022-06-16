mod house {
    pub const HOUSE_NUMBER: u32 = 56;

    pub mod bedroom1 {
        pub fn is_light_on() -> bool {
            false
        }
    }

    pub mod bedroom2 {
        pub fn is_light_on() -> bool {
            true
        }
    }
}

use crate::house::{bedroom1, bedroom2, HOUSE_NUMBER};

fn main() {
    println!("Hello, world!");
    println!("{}", HOUSE_NUMBER);
    println!("{}", bedroom1::is_light_on());
    println!("{}", bedroom2::is_light_on());
}
