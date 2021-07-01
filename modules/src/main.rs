mod house {
    const HOUSE_NUMBER: u32 = 56;

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

use crate::house::{bedroom1, bedroom2};

fn main() {
    println!("Hello, world!");
    println!("{}", bedroom1::is_light_on());
    println!("{}", bedroom2::is_light_on());
}
