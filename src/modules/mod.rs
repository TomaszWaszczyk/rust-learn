// https://www.cs.brandeis.edu/~cs146a/rust/doc-02-21-2015/book/crates-and-modules.html

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
