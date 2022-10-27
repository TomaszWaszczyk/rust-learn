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
